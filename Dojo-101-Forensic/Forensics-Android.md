# Android Forensics

## Ressources

* [ADB](https://developer.android.com/tools/releases/platform-tools)

* [MVT Mobile Verification Toolkit](https://github.com/mvt-project/mvt)

* [ALEAPP](https://github.com/abrignoni/ALEAPP)

## Structure du Système de fichier

| Chemin | Description |
|--------|-------------|
| /system | system et applications préinstallées|
| /data | données utilisateurs, dont contact et messages |
| /sdcard | données externes ou internes, fichiers multimédias et documents |
| /cache | Fichiers temporaires |
| /data | Données des Applications |


## Procedure

Device allumé avec connaissance des Credentials

1. Mode avions, désactivation Wifi et Bluetooth

2. Activation du débogage USB

4. Étendre le Lock Timeaout

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

Pour spécifier une date `adb logcat -d -T "2024-11-29 16:00:00.000"`

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

### Pull de fichiers

téléchargement des documents utilisateur

```powershell
.\adb.exe pull /sdcard sdcard
```


### Vérifier l'absence de rootage

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

> Ce type de backup est déprécié par Google.

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

## Analyse des applications

* Peut se faire localement avec `Google Play Protect` depuis le Play Store

* Revue des permissions localement via `Paramètres` -> `Applications` et `Paramètres` -> `Sécurité et Confidentialité`, mieux vaut privilégier `adb`.

* Signatures des Apps ou Fichiers suspects : à mettre sur `VirusTotal` une fois l'APK téléchargée via `adb`

```powershell
.\adb.exe pull /data/app/chemin/base.apk base.apk
```


## Compte google

* [google security checkup](myaccount.google.com/security-checkup)

* [Support Google](https://support.google.com/accounts/answer/6294825?)


## Pour aller plus loin



### Recherche d'IoCs

Lorsqu'on sait ce que l'on cherche, trouver les IoCs Correspondants

```sh
mvt-android check-backup --iocs ~/iocs/malware.stix2 /path/to/android/backup/
```

### Misc

from [@PraveenAdithya](https://github.com/PraveenAdithya/Android-Forensics-Cheatsheet) et complété

```txt
Android-Forensics-Cheatsheet

Android forensic artifacts cheat sheet :

.\data\system\users%USERNUMBER%\settings_secure.xml - Android ID, Bluetooth name, Bluetooth address

..\data\system\usagestats%USERNUMBER%\version - OS version, Build codename, Build version

..\data\drm\pvt\ahrh - IMEI

..\data\user_de%USERNUMBER%\com.android.providers.telephony\databases\telephony.db - Display name, ICCID, IMSI, Country (SIM card details)

..\data\misc\bootstat\factory_reset - Factory reset time (UTC)

..\data\misc\bootstat\last_boot_time_utc - Last boot time (UTC)

..\data\misc\adb\adb_keys. - host connects to the device through adb.

..\data\system_ce%USERNUMBER%\accounts_ce.db - users’ application account details, including login credentials, account IDs, authentication tokens, and more

..\data\system_de%USERNUMBER%\accounts_de.db - users’ application account details, including login credentials, account IDs, authentication tokens, and more

..\data\user%USERNUMBER%\com.google.android.apps.turbo\shared_prefs\app_usage_stats.xml - hardware and software usage statistics.

.\data\data\com.google.android.apps.wellbeing\databases\app_usage - Digital Wellbeing service that collects usage statistic

..\data\data\com.samsung.android.forest\databases\dwbCommon.db - Digital Wellbeing service Samsung

.\data\data\com.android.vending\databases\frosting.db - application is installed or updated

..\data\data\com.android.vending\databases\suggestions.db - Google Play Store searches

..\data\data\com.WhatsApp\databases\msgstore.db - WhatsApp stores account data

..\data\data\com.WhatsApp\files - WhatsApp encrypts and decrypts database backups with a key

..\Android\media\com.whatsapp\WhatsApp\Media. - WhatsApp Media files

/data/datacom.instagram.android/shared_prefs/com.instagram.android_preferences.xml - Instagram -User ID,Username,account type,user account accesstime, biography, profile photo

/data/datacom.instagram.android/shared_prefs/_userBootstrapService.xml - Instagrams followers,following, close friends list

/data/data/com.android.providers.settings/databases/settings.db - secrets, dont salt

/data/system/password.key -	secret lockScreen

/data/data/com.android.chrome/databases/webview.db - Chrome history

/data/data/com.whatsapp/ - wa.db et msgstore.db contient les messages WhattsAp

/mnt/sdcard/WhatsApp/.Share/ - Fichiers partagés WhattsApp

```