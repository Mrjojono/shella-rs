# Todo — shella-rs

Roadmap du projet. Coche la case quand c'est fait, l'ordre des sections = l'ordre de priorité.

## Terminé

- [x] Commande `help` (affichage de l'aide depuis `src/utils/help.txt`)

## En cours

- [ ] Implémenter les autres commandes essentielles du shell

## À faire

- [ ] Logique de création de projet (bootstrap du template, `git init`, etc.)
- [ ] Interface de création de projet
- [ ] Gestion de git dans le terminal
- [ ] Jeu de frappe (monkey type) dans le terminal
- [ ] Interface graphique (TUI) du CLI

## En continu

- [ ] Améliorer l'interface (TUI) du CLI

## Idées

- [ ] Réfléchir à d'autres fonctionnalités

---

## Convention de suivi dans le code

Le roadmap ci-dessus vit dans ce fichier. Les morceaux **non encore écrits** se signalent
directement dans le code source, pas ici :

| Marqueur              | Signification                                        |
| --------------------- | ---------------------------------------------------- |
| `todo!()`             | « je vais l'implémenter » — à supprimer une fois fait |
| `unimplemented!()`    | « je ne l'implémenterai pas » (exclu définitivement)  |
| `unreachable!()`      | « statiquement impossible » (à remplacer par un match exhaustif) |
| `// TODO:`            | note de planification, sans impact sur le runtime      |
| `// FIXME:`           | bug connu                                            |

`clippy::todo` et `clippy::unimplemented` sont activés dans `Cargo.toml` ([lints]), donc
`cargo clippy` signale automatiquement tout `todo!()` oublié avant commit.
