## AutoCompletion Pour linux

#### Version: `0.1.0-dev.1` - **Note : L'application est encore instable, il est donc recommandé d'être indulgent, l'application risque de recevoir des mises à jour fréquemment**

Pour Windows, L'application n'est pas encore supportée mais bientôt.

Une fois installé, vous pouvez l’exécuter avec la commande $ autocompletion lorsque vous allez écrire un mot, peu importe ou sur votre ordinateur, vous pouvez le compléter avec le raccourci `ctrl + Espace` 
*Par exemple, si vous écrivez `bonj` et que vous faites le raccourci, le mot sera automatiquement complété avec : `bonjour`*

L’auto-complétion va continuer de s’exécuter en arrière-plan  et va apprendre avec les mots que vous écrivez continuellement.Pour des raisons de sécurité et de confidentialité, les mots que vous écrivez ne sont pas ajoutés a la base de données. Seulement la fréquence d'utilisation des mots déjà présents sera ajustée afin de proposer de meilleures propositions dans le futur. **Note: aucune donnée n'est envoyée au serveur, tout reste en local**

### Installation
```bash
curl https://raw.githubusercontent.com/Heficience/autocompletion/master/install.sh | sh
```

### Contributions
- andronedev (Créateur)
<a href="https://github.com/Heficience/autocompletion/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Heficience/autocompletion" />
</a>


### Remerciements
- [Rust](https://rust-lang.com/) est le langage de programmation utilisé pour ce projet.
