# Forensic

[Interpol](https://www.interpol.int/content/download/16243/file/Guidelines_to_Digital_Forensics_First_Responders_V7.pdf)

## Si machine allumée

1. Isolation réseaux

2. Capture de la RAM

3. Déchiffrement ou récupération de la clé de chiffrement

4. Arret et copie du disque

## Copie du Disque

* **Device to Device (clone)** : This can be performed by obtaining an exact bit-by-bit replica of an original device in another - previously wiped - device with equal or greater capacity.

* **Device to File (image)** : This can be performed by generating one or more files that contain, linked together, an identical copy of the original device. The most widespread is `dd` (`raw`) or `E01` formats

## Collecte d'artefact

[FASTIR](https://github.com/SekoiaLab/Fastir_Collector)


### Misc

Montage read only :

```bash
mount  -o loop,ro,noexec img.dd  /mnt
```
> ne remplace pas duplicateur avec un blocker Hardware !

