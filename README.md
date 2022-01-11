## AutoCompletion Pour linux

#### Version: `0.1.0-dev.1` - *En cours de développement, il est recommandé de l'utiliser que pour des tests*

### Installation
```bash
# Lib:
sudo apt install libx11-dev libxdo-dev -y

curl -s https://api.github.com/repos/Heficience/autocompletion/releases/latest | grep "browser_download_url" | cut -d : -f 2,3 | tr -d \" | wget -qi -

chmod +x autocompletion

./autocompletion

```

### Contributions
- andronedev (Créateur)
<a href="https://github.com/Heficience/autocompletion/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Heficience/autocompletion" />
</a>


### Remerciements
- [Rust](https://rust-lang.com/) est le langage de programmation utilisé pour ce projet.
