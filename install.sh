echo "AUTOCOMPLETION INSTALLATION"
echo "=========================="
echo "heficience.com"
echo "=========================="

echo "Installing dependencies..."
sudo apt install libx11-dev libxdo-dev -y
echo "Downloading latest version of autocompletion"
wget $(curl -s https://api.github.com/repos/Heficience/autocompletion/releases/latest | grep "autocompletion_linux" | awk '{print $2}' | sed 's|[\"\,]*||g'
autocompletion_linux)

# rm old autocompletion
echo "Removing old autocompletion.."
sudo rm -f /usr/bin/autocompletion
rm -rf ~/.autocompletion/
echo "Installing autocompletion.."
mkdir ~/.autocompletion/
mv autocompletion_linux ~/.autocompletion/autocompletion
chmod +x ~/.autocompletion/autocompletion

# create bash to run autocompletion
echo "Creating bash to run autocompletion"

echo "cd ~/.autocompletion/ && ./autocompletion" > ~/.autocompletion/autocompletion.sh
chmod +x ~/.autocompletion/autocompletion.sh

# create a symbolic link in /usr/bin
echo "Creating symbolic link in /usr/bin"
sudo ln -s ~/.autocompletion/autocompletion.sh /usr/bin/autocompletion


echo "Run autocompletion to start"
echo "$ autocompletion"
