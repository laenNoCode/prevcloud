the files are stored in a folder specified at instance spawn (in config file).
The files may be ciphered/deciphered on the user side using an asymmetric key system.
The user stores on the filesystem his public key and his private key ciphered using his password.
A key will be generated by the user to cipher each file and will be stored in a file in the folder dedicated to that.

User may be grouped later on.

Each user gets assigned a folder wich will be stored in the database.

The database contains the followind tables and fields

+-------+
| User  |
+-------+
| id    |
| name  |
| hash  |
| folder|
+-------+

+---------+
| session |
+---------+
|token    |
|user.id  |
|expiresat|
+---------+

+---------------+
|tobeRemoved    |
+---------------+
|user.id        |
|basefileName   |
|currentfilename|
+---------------+

+-------------+
|sharedLink   |
+-------------+
|path         |
|targetuser.id|
|baseuser.id  |
|expires      |
+-------------+

