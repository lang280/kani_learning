## Windows
### Universal ctags installation
ctags dependences installation `sudo apt install -y autoconf automake pkg-config`
clone ctags github repo `git clone https://github.com/universal-ctags/ctags.git`
```
cd ctags
./autogen.sh
./configure
make
sudo make install
```
verify installation: ```ctags --version```



## MacOS
### Universal ctags installation
`brew install universal-ctags`

## Kani installation
```
cargo install --locked kani-verifier
cargo kani setup
```