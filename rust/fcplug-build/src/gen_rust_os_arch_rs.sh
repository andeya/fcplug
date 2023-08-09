#!/bin/bash

output=$(rustc --print target-list)
output=$(echo "$output" | sed 's/\./_/g')

RustOS=()
RustArch=()

while IFS= read -r line; do
  IFS='-' read -ra a <<< "$line"
  if [[ ${#a[@]} -ge 3 ]]; then
    RustOS+=("${a[2]}")
  fi
  if [[ ${#a[@]} -ge 1 ]]; then
    RustArch+=("${a[0]}")  # 将第一个元素添加到数组中
  fi
done <<< "$output"

unique_RustOS=$(printf "%s\n" "${RustOS[@]}" | sort -u)
unique_RustArch=$(printf "%s\n" "${RustArch[@]}" | sort -u)

rs_file="rust_os_arch_gen.rs"
rm -f "$rs_file"

echo "#![allow(non_camel_case_types)]" > $rs_file

echo "" >> $rs_file

echo "#[derive(strum::AsRefStr, strum::EnumString)]" >> $rs_file
echo "pub enum RustOS {" >> $rs_file
for os in "${unique_RustOS[@]}"; do
  echo "$os" | sed 's/^[0-9].*/#\[strum(serialize = "&")\]_&/; s/^/    /; s/$/,/' >> $rs_file
done
echo "}" >> $rs_file

echo "" >> $rs_file

echo "#[derive(strum::AsRefStr, strum::EnumString)]" >> $rs_file
echo "pub enum RustArch {" >> $rs_file
for arch in "${unique_RustArch[@]}"; do
  echo "$arch" | sed 's/^[0-9].*/#\[strum(serialize = "&")\]_&/; s/^/    /; s/$/,/' >> $rs_file
done
echo "}" >> $rs_file
