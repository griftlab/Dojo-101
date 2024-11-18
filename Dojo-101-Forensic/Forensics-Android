# Android Forensics

## Ressources

* [ADB](https://developer.android.com/tools/releases/platform-tools)

* [MVT Mobile Verification Toolkit](https://github.com/mvt-project/mvt)

* [ALEAPP](https://github.com/abrignoni/ALEAPP)

## Structure du Système de fichier

| Chemin | Description |
|--------|-------------|
| /system | system et applications pré-installées|
| /data | données utilisateurs, dont contact et messages |
| /sdcard | données externe ou internes, fichiers multimedia et documents |
| /cache| Fichiers temporaires |
| /data/data/com.android.providers.settings/databases/settings.db | secrets, dont salt |
| /data/system/password.key | secret lockScreen |

## Procedure

Device allumé avec connaissance des Credentials

1. Mode avions, désactivation Wifi et Bluetooth

2. Activation du deboggage USB

4. Etendre le Lock Timeaout

5. Collecter les identifiants

6. Transport au Lab

## Android mode Dev + debug USB

1. `Paramètres` -> `à propos du téléphone` -> `Numéro de build`*4

2. `Paramètres` -> `Systèmes` -> `Optins pour les développeurs` -> `Debogage USB`

## Adb

### lancement du serveur adb : 

```powershell
.\adb.exe version
.\adb.exe start-server
```

### devices

```powershell
.\adb.exe devices
.\adb.exe get-serialno
.\adb.exe get-state
```

Si `unauthorized`, autorisation sur le téléphone à valider

### Dump Système

```powershell
.\adb.exe shell dumpsys > Dumpsys.txt
```

### processus

```powershell
.\adb.exe shell ps > ps.txt
```

### Logs

```powershell
.\adb.exe shell logcat -d > Logs.txt
.\adb.exe shell logcat 
```

### packages

```powershell
.\adb.exe shell pm list packages
.\adb.exe shell pm list packages -f > packages.txt
```

### permissions

```powershell
.\adb.exe shell pm list permissions > permissions.txt
.\adb.exe shell "dumpsys package packagename | grep permission"
```

### contacts

```powershell
.\adb.exe shell content query --uri content://com.android.contacts/contacts > contact.txt
```

### SMS/MMS :

```powershell
.\adb.exe shell content query --uri content://sms > sms.txt
.\adb.exe shell content query --uri content://mms > mms.txt
```

### montage

```powershell
.\adb.exe shell mount
.\adb.exe shell ls /mnt
```

### Réseaux

```powershell
.\adb.exe shell dumpsys connectivity
.\adb.exe shell dumpsys telephony.registry
```

### parcourir les fichiers

```powershell
.\adb.exe shell ls /
```

### Pull / Push de fichiers

exemple avec un téléchargement d'APK

```powershell
.\adb.exe pull /data/app/chemin/base.apk base.apk
```

### Verifier l'absence de rootage

```powershell
.\adb.exe root
.\adb.exe shell id
.\adb.exe shell su
.\adb.exe shell "su -c whoami"
```

### Fastboot pour vérifier le `secure boot` et autres informations

```sh
adb reboot bootloader
fastboot oem device-info
fastboot reboot
```



### Arrêt du serveur adb : 

```powershell
.\adb.exe kill-server
```


## Backup pour comparaison aux Iocs

### Via Google

`Paramètres` -> `Système` -> `Sauvegarde`, ensuite visible sur [Google](https://drive.google.com/drive/backups)

### Via adb

> Ce type de backup est déprecié par Google.

Backup des SMS

```sh
adb backup -nocompress com.android.providers.telephony
```

Backup complète

```sh
adb backup -nocompress -all
```

### Extraction des backups

* via [ABE](https://github.com/nelenkov/android-backup-extractor) : 

```powershell
java -jar .\abe-3e9a273.jar unpack .\backup.ab backup.tar
```

* via l'outil `MVT` :

```sh
mvt-android check-backup --output /path/to/results/ /path/to/backup.ab`
```

## Recherche d'IoCs

```sh
mvt-android check-backup --iocs ~/iocs/malware.stix2 /path/to/android/backup/
```

## Analyse des applications

* Peut se faire localement avec `Google Play Protect` depuis le Play Store

* Revue des permissions localement via `Paramètres` -> `Applications`, mieux vaut privilegier adb.

* Signatures des Apps ou Fichiers suspects : `VirusTotal`


## Compte google

* [google security checkup](myaccount.google.com/security-checkup)

* [Support Google](https://support.google.com/accounts/answer/6294825?)

## WhatsApp

* `wa.db` et `msgstore.db` : `/data/data/com.whatsapp/`  
* Shared Files : `/mnt/sdcard/WhatsApp/.Share/`