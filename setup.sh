# install basic tools
apt-get -y install dos2unix python-software-properties vim

# install Rust nightly
add-apt-repository ppa:hansjorg/rust
apt-get update
apt-get install rust-nightly

# install vim syntax highlighting
wget -nv https://raw.githubusercontent.com/mozilla/rust/master/src/etc/vim/syntax/rust.vim -O /usr/share/vim/vim73/syntax/rust.vim
echo 'au BufNewFile,BufRead *.rs set filetype=rust' >> /usr/share/vim/vimrc

# install run-rust tool
run_rust_path=/usr/local/bin/run-rust
cp /vagrant/run-rust "$run_rust_path"
dos2unix "$run_rust_path" # in case host is Windows
chmod +x "$run_rust_path"
