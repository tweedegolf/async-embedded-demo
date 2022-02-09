
echo "Enter install location for openocd 0.11.0-3..."
read;
INSTALL_PATH=$REPLY
INSTALL_PATH=/home/henkdieter/bin
echo "Installing openocd 11 at $INSTALL_PATH..."
wget -qO - https://github.com/xpack-dev-tools/openocd-xpack/releases/download/v0.11.0-3/xpack-openocd-0.11.0-3-linux-x64.tar.gz \
    | tar xzvf - -C $INSTALL_PATH xpack-openocd-0.11.0-3

PATH_EXPORT="export PATH=\"$INSTALL_PATH:\$PATH\""
grep -qxF "$PATH_EXPORT" $HOME/.bashrc || echo $PATH_EXPORT >> $HOME/.bashrc

echo "Installed openocd 11 at $INSTALL_PATH. To uninstall, run:"
echo ""
echo "rm -r $INSTALL_PATH/xpack-openocd-0.11.0-3"
echo ""
echo "NOTE: you may get the following error when starting openocd:"
echo "openocd11: error while loading shared libraries: libudev.so.0: cannot open shared object file: No such file or directory"
echo ""
echo "In that case, you may want to run:"
echo "sudo ln -sf /lib/x86_64-linux-gnu/libudev.so.1 /lib/x86_64-linux-gnu/libudev.so.0"
echo ""
echo "Open a new terminal, or run source $HOME/.bashrc to use openocd!"
echo ""