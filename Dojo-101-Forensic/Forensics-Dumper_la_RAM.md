# Dump de la RAM

## Windows

* [Winpmem](https://github.com/Velocidex/c-aff4/releases) Compatible avec Volatility.

```powershell
C:\winpmem_v3.3.rc3.exe --output memdump.raw --format raw --volume_format raw
```

* `Dumpit.exe`

* `NotMyFault.exe` 

* `Windows` : système -> avancé -> sauvegarde et restauration -> vidage de la RAM

* [process hacker](https://processhacker.sourceforge.io/) On peut visualiser les process mais aussi directement lire les strings en ram etc : `click droit sur le process -> proprietés -> memory -> strings`

* `Sysinternals: procdump` 

```powershell
C:\Users\Chase\Documents>.\procdump.exe -accepteula -ma <pid> 
.\procdump.exe -accepteula -ma 6800
```

## Linux / Android

* [LiME (archived)](https://github.com/504ensicsLabs/LiME)
