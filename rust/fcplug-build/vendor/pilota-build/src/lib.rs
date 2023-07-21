#![doc(
html_logo_url = "https://github.com/cloudwego/pilota/raw/main/.github/assets/logo.png?sanitize=true"
)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![allow(clippy::mutable_key_type)]
#![allow(warnings)]

use std::{path::PathBuf, sync::Arc};

pub use codegen::{
    Codegen, protobuf::ProtobufBackend, traits::CodegenBackend,
};
use db::{RirDatabase, RootDatabase};
use middle::{
    context::{CollectMode, ContextBuilder, Mode, tls::CONTEXT, WorkspaceInfo},
    rir::NodeKind,
    type_graph::TypeGraph,
};
pub use middle::{
    adjust::Adjust,
    context::{Context, SourceType},
    rir, ty,
};
use parser::{Parser, ParseResult, protobuf::ProtobufParser, thrift::ThriftParser};
use plugin::{
    AutoDerivePlugin, BoxedPlugin, EnumNumPlugin, ImplDefaultPlugin, PredicateResult,
    WithAttrsPlugin,
};
pub use plugin::{BoxClonePlugin, ClonePlugin, Plugin};
use resolve::{Resolver, ResolveResult};
use salsa::Durability;
pub use symbol::{DefId, IdentName};
pub use symbol::Symbol;
pub use tags::TagId;

mod util;

pub mod codegen;
pub mod db;
pub(crate) mod errors;
mod fmt;
mod index;
pub mod ir;
mod middle;
pub mod parser;
mod resolve;
mod symbol;
pub mod tags;
// mod dedup;
pub mod plugin;

pub trait MakeBackend: Sized {
    type Target: CodegenBackend;
    fn make_backend(self, context: Context) -> Self::Target;
}


pub struct MkProtobufBackend;

impl MakeBackend for MkProtobufBackend {
    type Target = ProtobufBackend;

    fn make_backend(self, context: Context) -> Self::Target {
        ProtobufBackend::new(context)
    }
}

pub struct Builder<MkB, P> {
    source_type: SourceType,
    mk_backend: MkB,
    parser: P,
    plugins: Vec<Box<dyn Plugin>>,
    ignore_unused: bool,
    touches: Vec<(std::path::PathBuf, Vec<String>)>,
    change_case: bool,
}

impl<MkB> Builder<MkB, ThriftParser> {
    pub fn thrift_with_backend(mk_backend: MkB) -> Self {
        Builder {
            source_type: SourceType::Thrift,
            mk_backend,
            parser: ThriftParser::default(),
            plugins: vec![
                Box::new(WithAttrsPlugin(Arc::from(["#[derive(Debug)]".into()]))),
                Box::new(ImplDefaultPlugin),
                Box::new(EnumNumPlugin),
            ],
            touches: Vec::default(),
            ignore_unused: true,
            change_case: true,
        }
    }
}

impl<MkB> Builder<MkB, ProtobufParser> {
    pub fn protobuf_with_backend(mk_backend: MkB) -> Self {
        Builder {
            source_type: SourceType::Protobuf,
            mk_backend: mk_backend,
            parser: ProtobufParser::default(),
            plugins: vec![
                Box::new(WithAttrsPlugin(Arc::from(["#[derive(Debug)]".into()]))),
                Box::new(ImplDefaultPlugin),
                Box::new(EnumNumPlugin),
            ],
            touches: Vec::default(),
            ignore_unused: true,
            change_case: true,
        }
    }
}

impl<MkB, P> Builder<MkB, P>
    where
        P: Parser,
{
    pub fn include_dirs(mut self, include_dirs: Vec<PathBuf>) -> Self {
        self.parser.include_dirs(include_dirs);
        self
    }
}

impl<MkB, P> Builder<MkB, P> {
    pub fn with_backend<B: MakeBackend>(self, mk_backend: B) -> Builder<B, P> {
        Builder {
            source_type: self.source_type,
            mk_backend,
            parser: self.parser,
            plugins: self.plugins,
            ignore_unused: self.ignore_unused,
            touches: self.touches,
            change_case: self.change_case,
        }
    }

    pub fn plugin<Plu: Plugin + 'static>(mut self, p: Plu) -> Self {
        self.plugins.push(Box::new(p));

        self
    }

    pub fn change_case(mut self, change_case: bool) -> Self {
        self.change_case = change_case;
        self
    }

    /**
     * Don't generate items which are unused by the main service
     */
    pub fn ignore_unused(mut self, flag: bool) -> Self {
        self.ignore_unused = flag;
        self
    }

    /**
     * Generate items even them are not used.
     *
     * This is ignored if `ignore_unused` is false
     */
    pub fn touch(
        mut self,
        item: impl IntoIterator<Item=(PathBuf, Vec<impl Into<String>>)>,
    ) -> Self {
        self.touches.extend(
            item.into_iter()
                .map(|s| (s.0, s.1.into_iter().map(|s| s.into()).collect())),
        );
        self
    }
}

pub enum Output {
    Workspace(PathBuf),
    File(PathBuf),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IdlService {
    pub path: PathBuf,
    pub config: serde_yaml::Value,
}

impl IdlService {
    pub fn from_path(p: PathBuf) -> Self {
        IdlService {
            path: p,
            config: Default::default(),
        }
    }
}

impl<MkB, P> Builder<MkB, P>
    where
        MkB: MakeBackend + Send,
        MkB::Target: Send,
        P: Parser,
{
    pub fn compile(
        self,
        services: impl IntoIterator<Item=impl AsRef<std::path::Path>>,
        out: Output,
    ) {
        let services = services
            .into_iter()
            .map(|path| IdlService {
                config: serde_yaml::Value::default(),
                path: path.as_ref().to_owned(),
            })
            .collect();

        self.compile_with_config(services, out)
    }

    pub fn compile_with_config(mut self, services: Vec<IdlService>, out: Output) {
        let _ = tracing_subscriber::fmt::try_init();

        let mut db = RootDatabase::default();
        self.parser.inputs(services.iter().map(|s| &s.path));
        let ParseResult {
            files,
            input_files,
            file_ids_map,
        } = self.parser.parse();
        db.set_file_ids_map_with_durability(Arc::new(file_ids_map), Durability::HIGH);

        let ResolveResult { files, nodes, tags } = Resolver::default().resolve_files(&files);

        // discard duplicated items
        // let mods = nodes
        //     .iter()
        //     .into_group_map_by(|(_, node)|
        // files.get(&node.file_id).unwrap().package.clone());

        // for (_, m) in mods {
        //     m.iter().unique_by(f);
        // }

        db.set_files_with_durability(Arc::new(files), Durability::HIGH);
        let items = nodes.iter().filter_map(|(k, v)| {
            if let NodeKind::Item(item) = &v.kind {
                Some((*k, item.clone()))
            } else {
                None
            }
        });

        let type_graph = Arc::from(TypeGraph::from_items(items));
        db.set_type_graph_with_durability(type_graph, Durability::HIGH);
        db.set_nodes_with_durability(Arc::new(nodes), Durability::HIGH);
        db.set_tags_map_with_durability(Arc::new(tags), Durability::HIGH);

        let mut input = Vec::with_capacity(input_files.len());
        for file_id in &input_files {
            let file = db.file(*file_id).unwrap();
            file.items.iter().for_each(|def_id| {
                if matches!(&*db.item(*def_id).unwrap(), rir::Item::Service(_)) {
                    input.push(*def_id)
                }
            });
        }
        db.set_input_files_with_durability(Arc::new(input_files), Durability::HIGH);

        let mut cx = ContextBuilder::new(
            db,
            match out {
                Output::Workspace(dir) => Mode::Workspace(WorkspaceInfo {
                    dir,
                    location_map: Default::default(),
                }),
                Output::File(p) => Mode::SingleFile { file_path: p },
            },
            input,
        );

        cx.collect(if self.ignore_unused {
            CollectMode::OnlyUsed {
                touches: self.touches,
            }
        } else {
            CollectMode::All
        });

        let cx = cx.build(Arc::from(services), self.source_type, self.change_case);

        cx.exec_plugin(BoxedPlugin);

        cx.exec_plugin(AutoDerivePlugin::new(
            Arc::from(["#[derive(PartialOrd)]".into()]),
            |ty| {
                let ty = match &ty.kind {
                    ty::Vec(ty) => ty,
                    _ => ty,
                };
                if matches!(ty.kind, ty::Map(_, _) | ty::Set(_)) {
                    PredicateResult::No
                } else {
                    PredicateResult::GoOn
                }
            },
        ));

        cx.exec_plugin(AutoDerivePlugin::new(
            Arc::from(["#[derive(Hash, Eq, Ord)]".into()]),
            |ty| {
                let ty = match &ty.kind {
                    ty::Vec(ty) => ty,
                    _ => ty,
                };
                if matches!(ty.kind, ty::Map(_, _) | ty::Set(_) | ty::F64 | ty::F32) {
                    PredicateResult::No
                } else {
                    PredicateResult::GoOn
                }
            },
        ));

        self.plugins.into_iter().for_each(|p| cx.exec_plugin(p));

        std::thread::scope(|scope| {
            let pool = rayon::ThreadPoolBuilder::new();
            let pool = pool
                .spawn_handler(|thread| {
                    let mut builder = std::thread::Builder::new();
                    if let Some(name) = thread.name() {
                        builder = builder.name(name.to_string());
                    }
                    if let Some(size) = thread.stack_size() {
                        builder = builder.stack_size(size);
                    }

                    let cx = cx.clone();
                    builder.spawn_scoped(scope, move || {
                        CONTEXT.set(&cx, || thread.run());
                    })?;
                    Ok(())
                })
                .build()?;

            pool.install(move || {
                let cg = Codegen::new(self.mk_backend.make_backend(cx));
                cg.gen().unwrap();
            });

            Ok::<_, rayon::ThreadPoolBuildError>(())
        })
            .unwrap();
    }
}

mod test;
