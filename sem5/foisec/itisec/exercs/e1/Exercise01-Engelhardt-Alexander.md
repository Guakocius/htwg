# Exercise Sheet 01

## 1.

Amount of passwords: P = 26¹⁴ = 64,509,974,703,297,150,976 Passwords

### 1.1

Pt = 20000 P/second => 20,000 * 60 * 60 * 24 = 1,728,000,000 P/day
64,509,974,703,297,150,976 Passwords / 1,728,000,000 P/day = 37,332,161,286.630295 days ~ 37,332,161,287 days

### 1.2

Pp (Passwords popular) = 1,000 => H = 5,000 * 0.9 = 4,500 Passwords => Ppavg = 4,500 P / 1000 P = 4.5 P / 1000P
Pt = 2,000 P/second
90 Accounts = 90/4.5 = 20 Popular passwords

N = 20 * 5,000 = 100,000 Passwords

T = 100,000 P / 2,000 P/second = 50 seconds

### 1.3

P = 90%
S = N/1000 => P(S) = 0.9(S) = 0.9(N/1000) <= (less or equal) 0.2 => N <= (less or equal) (0.2*1000)/0.9
=> N <= (less or equal) 222.222 ~ 222 passwords
A: The attacker may try 222 passwords until their probability of having a success exceeds 80%

## 2.

### 2.1

Three apps:
- WhatsApp => end-to-end encrypted digital communication; Permissions:
    - Location
    - Contacts
    - Photos
    - Microphone
    - Camera
    - Face ID
    - Apple Intelligence & Siri
    - Search function
    - Notifications
    - Live Activities
    - Background Updates
    - Mobile Data
- Instagram => social medium to share photos and/or videos and interacting with them => connect with others; Permissions:
    - Contacts
    - Photos
    - Microphone
    - Camera
    - Apple Intelligence & Siri
    - Search function
    - Notifications
    - Live Activities
    - Background Updates
    - Mobile Data
- Reddit => social medium to share niche interests to a community; Permissions:
    - Apple Intelligence & Siri
    - Search function
    - Notifications
    - Live Activities
    - Background Updates
    - Mobile Data

These lists are rather capability lists than access control lists because they don't list up rules
of access for certain users or systems but list a list of permissions on which the app has
the ability to perform their actions.

### 2.2

Role based access is the ability for administrators to create different kinds of users and system
administrators, such that a user gets a role and can only do what the role permits for the tasks
required. For example, an ordinary student may only view and complete their assignments and view 
lecture material, but may not add new lecture slides to the moodle course.

## 3.

## 3.1

Ensures code integrity by whitelisting i.e. let only trusted applications be run.

## 3.2

Yes.
binary on whitelist -> executable, else not.

## 3.3

No. Locked. Is a matter of permissions from ACLs.

## 3.4

- hash value of binary
- publisher of digitally signed binaries
- file path
- file name

## 3.5

Logs instead of restricting users on using applications.

## 3.6

Important ID: **3076**, **3077** left audit mode and went into Real Mode -> Ok
Windows Event Viewer (Ereignisanzeige)
Anwendungs- und Dienstprotokolle
Code Integrity - Operational
