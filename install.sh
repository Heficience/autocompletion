echo "AUTOCOMPLETION INSTALLATION"
echo "=========================="
echo "heficience.com"
echo "=========================="

echo "Installing dependencies..."
sudo apt install libx11-dev libxdo-dev -y
echo "Downloading latest version of autocompletion"
wget $(curl -s https://api.github.com/repos/Heficience/autocompletion/releases/latest | grep "autocompletion_linux" | awk '{print $2}' | sed 's|[\"\,]*||g'
autocompletion_linux)
user_path=$(echo $HOME)
install_path=$user_path/.autocompletion
# rm old autocompletion
echo "Removing old autocompletion.."
sudo rm -f /usr/bin/autocompletion
rm -rf $install_path/
echo "Installing autocompletion.."
mkdir $install_path/
mv autocompletion_linux $install_path/autocompletion
chmod +x $install_path/autocompletion

# create bash to run autocompletion
echo "Creating bash to run autocompletion"

echo "
#!/bin/sh
# Path: autocompletion
path=$install_path/
cd \$path
args = \"\$@\"
./autocompletion \$args
" > $install_path/autocompletion.sh
chmod +x $install_path/autocompletion.sh

# create a symbolic link in /usr/bin
echo "Creating symbolic link in /usr/bin"
sudo ln -s $install_path/autocompletion.sh /usr/bin/autocompletion


echo "Run autocompletion to start"
echo "$ autocompletion"
