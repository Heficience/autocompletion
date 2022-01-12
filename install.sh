sudo apt install libx11-dev libxdo-dev -y
wget $(curl -s https://api.github.com/repos/Heficience/autocompletion/releases/latest | grep "autocompletion_linux" | awk '{print $2}' | sed 's|[\"\,]*||g'
autocompletion_linux)

mv autocompletion_linux ~/.autocompletion/autocompletion
chmod +x ~/.autocompletion/autocompletion
# create a symbolic link in /usr/bin
sudo ln -s ~/.autocompletion/autocompletion /usr/bin/autocompletion


echo "Run autocompletion to start"
echo "$ autocompletion"
