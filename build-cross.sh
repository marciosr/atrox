export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=/usr/x86_64-w64-mingw32/lib/pkgconfig
export GTK_INSTALL_PATH=/usr/x86_64-w64-mingw32
cargo build --target=x86_64-pc-windows-gnu --release
