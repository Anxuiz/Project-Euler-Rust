# install basic tools
apt-get -y install python-software-properties vim

# install Rust nightly
add-apt-repository ppa:hansjorg/rust
apt-get update
apt-get install rust-nightly

# install vim syntax highlighting
wget -nv https://raw.githubusercontent.com/mozilla/rust/master/src/etc/vim/syntax/rust.vim -O /usr/share/vim/vim73/syntax/rust.vim
echo 'au BufNewFile,BufRead *.rs set filetype=rust' >> /usr/share/vim/vimrc

# install run-rust tool
run_rust_path=/usr/local/bin/run-rust
cat > "$run_rust_path" <<EOF
#!/bin/bash
if [ $# -ne 1 ]; then
    echo "Usage: $0 <source>"
    exit 1
fi

source="$1"
filename=$(basename "$source")
output="/tmp/${filename%.rs}"

rustc "$source" -o "$output"
if [ $? -ne 0 ]; then
    exit 1
fi

"$output"
rm "$output"
EOF
chmod +x "$run_rust_path"
