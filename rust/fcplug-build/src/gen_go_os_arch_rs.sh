#!/bin/bash

json=$(go tool dist list -json);

# 提取GOOS和GOARCH字段的值
os_values=$(echo "$json" | grep -oE '"GOOS": "[^"]+"' | cut -d'"' -f4)
arch_values+="$(echo "$json" | grep -oE '"GOARCH": "[^"]+"' | cut -d'"' -f4)"

# 去重并生成rust的enum
unique_os_values=$(echo "$os_values" | tr ' ' '\n' | sort -u)
enum_os_values=$(echo "$unique_os_values" | sed 's/^[0-9].*/#\[strum(serialize = "&")\]_&/; s/^/    /; s/$/,/')
unique_arch_values=$(echo "$arch_values" | tr ' ' '\n' | sort -u)
enum_arch_values=$(echo "$unique_arch_values" | sed 's/^[0-9].*/#\[strum(serialize = "&")\]_&/; s/^/    /; s/$/,/')


rs_file="go_os_arch_gen.rs"
rm -f "$rs_file"

echo "#![allow(non_camel_case_types)]" > $rs_file

echo "" >> $rs_file

# 输出rust的enum定义
echo "#[derive(strum::AsRefStr, strum::EnumString)]" >> $rs_file
echo "pub enum GoOS {" >> $rs_file
echo "$enum_os_values" >> $rs_file
echo "}" >> $rs_file

echo "" >> $rs_file

echo "#[derive(strum::AsRefStr, strum::EnumString)]" >> $rs_file
echo "pub enum GoArch {" >> $rs_file
echo "$enum_arch_values" >> $rs_file
echo "}" >> $rs_file
