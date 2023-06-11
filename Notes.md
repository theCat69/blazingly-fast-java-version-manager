# Notes to myself 

The idea behind this project is to easyly change java version in current prompt on any OS

## load configuration from a file and represent configuration by struct
Should look something like this 
```rust 
struct Config {
    user_jdk_folder: Path,
    java_versions: HashMap<JavaVersion>,
}

struct JavaVersion {
    home_folder: Path,
    installed: bool,
}
```
I did my own thing becasue no lib was doing what i needed


## Cli

- clap
- proposer de donner son user_jdk_folder à la première utilisation
- proposer un argument pour changer la version de java
- proposer un argument pour changer son user_jdk_folder
- proposer un argument pour reset complement la config
- proposer un argument pour configurer le dossier d'installation de java (etre souple avec java.exe, /bin ou /JAVA_HOME) et bien vérifier la version de jjava 

## How does this work

- modify JAVA_HOME and PATH env variables
- if user want export 
- Si la version de java n'est pas installé alors renvoyé une aide spécifique

## Bonus

- si la version n'éxiste pas sur le disk alors proposer de la télécharger (proposer plusieurs distribution ?)
- permettre d'avoir plusieurs distro et de sauvegarder des alias
- changer la configuration par quelque chose qu'un humain peut lire facilement (json ?)
- si aucun argument n'est fourni proposer les versions dans une liste et choisir par numéro
- tester toutes les distribution depuis java6
- permit to use bfjvm as a global java version manager by changing permanently java version for the user

## Tech note for git_bash

currently local change in bash prompt is not supported
to make this possible we could add something in the .bashrc to create two folder (with a random identifier) on init that will be deleted
when you close the prompt (use trap and just delete the folders). Those folders should be referenced by env variables that are specific for this task.
append one of this folder to the PATH and the other one as the value of JAVA_HOME for this prompt 
those folder will contains temporary symlink the will initially point to current JAVA_HOME and current java in PATH
on version change we will delete and redo those symlink.

