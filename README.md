# Limit Timer

Un timer simple et élégant qui s'affiche toujours au premier plan.

## Fonctionnalités

- Fenêtre transparente et déplaçable
- Toujours au premier plan
- Changement de couleur en rouge quand il reste moins de 15 minutes
- Format d'affichage : "XX h XX min XX s"

## Installation

1. Assurez-vous d'avoir Rust installé sur votre système
2. Clonez ce dépôt
3. Compilez le projet avec :
```bash
cargo build --release
```

## Utilisation

Pour lancer le timer avec une durée par défaut de 60 minutes :
```bash
./target/release/limit_timer
```

Pour spécifier une durée personnalisée (en minutes) :
```bash
./target/release/limit_timer 30  # Pour un timer de 30 minutes
```

## Configuration au démarrage de Windows

Pour lancer automatiquement le timer au démarrage de Windows :

1. Appuyez sur `Windows + R`
2. Tapez `shell:startup` et appuyez sur Entrée
3. Créez un raccourci vers l'exécutable dans ce dossier
4. Modifiez les propriétés du raccourci pour ajouter l'argument de durée si nécessaire

## Notes

- La fenêtre est déplaçable mais ne peut pas être redimensionnée
- Il n'y a pas de bouton pour fermer la fenêtre (utilisez le gestionnaire de tâches si nécessaire)
- Le timer passe en rouge quand il reste moins de 15 minutes 