# Homelab User Manager (HUM)

HUM is a utility for managing users in an LDAP-backed homelab
environment.  It intends to provide the following features:

* Talks directly to the LDAP server to create, modify, and delete users.
* Allows the administrator to declare operations in a configuration
file to match their own processes and procedures.
* Handles standard LDAP attributes and object types.
* Handles Samba LDAP attributes and NT password hashing.
* Log actions to standard cloud logging frameworks.
* Allows users to reset their passwords (administrator configuration
and optional administrator acceptance) from a web-based frontend.

