# Dome
Dome is a CLI password manager written in Rust that stores encrypted passwords safely inside the local vault. <br/>
It supports Windows and Linux operating systems.

## Commands

```dome --version``` - Shows current version of Dome. <br/>
```dome help``` - Shows list of all availible commands. <br/>
```dome list``` - Displays a list of domains saved in the vault. <br/>
```dome add <domain> -u <username>``` - Adds new domain to the vault. <br/>
```dome get <domain>``` - Displays username and decrypted password for given domain. <br/>
```dome remove <domain>``` - Removes domain from the vautl. <br/>
```dome gen <length>``` - Generates random password of the given length.

## Master password
Vault is accessed using master password. It is used to create and access every entry in the vault. Master password is not saved anywhere within the program and it is possible to use different master password for each entry.

Dome will prompt you to enter master password like this:
```
Enter master password: 
```

## Add new entry
Using the add command and providing domain name and username, you will be prompted to first input ```Master password``` and then password for the given domain.

```
> dome add gmail -u john.doe@gmail.com

Enter master password: ******
Repeat password: *****

Enter password for gmail: *****
Repeat password: *****

Password for gmail was saved in the vault.
+------------------+----------------------+
| Domain           | Username             |
+-----------------------------------------+
| gmail            | john.doe@gmail.com   |
+-----------------------------------------+
```

## List entries
With ```dome list``` command you can list all entries that are saved in your vault.

```
> dome list

+-----------------------+
| 0 | gmail             |
+-----------------------+
| 1 | yahoo             |
+-----------------------+
| 2 | github            |
+-----------------------+
```

## Show password for a domain
You access the saved password using ```dome get <domain>``` command.

```
> dome get gmail

Enter master password: ******

+------------------+----------------------+--------------------------+
| Domain           | Username             |                 Password |
+-----------------------------------------+--------------------------+
| gmail            | john.doe@gmail.com   |   my_very_secret_pwd123  |
+-----------------------------------------+--------------------------+

```

## Remove password from the vault
To remove entry from the vault, use ```dome remove <domain>```.

```
> dome remove gmail

Are you sure you want to remove gmail from the vault? [Y/n]: y

gmail was removed from the vault.

```
