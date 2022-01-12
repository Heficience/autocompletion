sudo apt install libx11-dev libxdo-dev -y
wget $(curl -s https://api.github.com/repos/Heficience/autocompletion/releases/latest | grep "autocompletion_linux" | awk '{print $2}' | sed 's|[\"\,]*||g'
autocompletion_linux)

sudo rm -f /usr/bin/autocompletion # Remove old autocompletion
sudo mv autocompletion_linux /usr/bin/autocompletion
sudo chmod +x /usr/bin/autocompletion


echo "Run autocompletion to start"
echo "$ autocompletion"
