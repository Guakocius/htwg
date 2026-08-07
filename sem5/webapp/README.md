# Flappy Bird -- Reimagined

**Team:** Guakocius

**Repository:** [https://github.com/Guakocius/htwg/tree/main/sem5/webapp](https://github.com/Guakocius/htwg/tree/main/sem5/webapp)

---

## Projektbeschreibung

**Flappy Bird -- Reimagined** ist eine moderne, typsichere Full-Stack-Webapplikation, die das ikonische Spielprinzip des Ur-Spiels aufgreift und um moderne Server-Komponenten erweitert. Die Anwendung kombiniert eine performante Client-seitige Game-Engine mit einer sicheren REST-API und persistenten HIghscore-Speicherung.

### Key Features

- **Classic Arcade Gameplay:** Reaktionsbasiertes Handling über die Leertaste oder Mausklicks mit kollisionsgenauer Physik.
- **Nutzerverwaltung & Authentifizierung:** Registrierung und Anmeldung mit JWT-basierten Sessions und Bcrypt-Passwort-Hashing.
- **Persistentes Leaderboard:** Serverseitig validierte Bestenliste (Top 10) zur Speicherung und Visualisierung von Rekorden.
- **Modernes UI/UX & Dynamic Theme:** Nahtloser Wechsel zwischen Light- und Darkmode für optimale Lese- und Spielbarkeit.
- **End-to-End Typsicherheit:** Durchgängiger Einsatz von TypeScript vom React-Frontend bis zum Express-Backend.

---

## Architekturübersicht

Die Anwendung basiert auf einer entkoppelten **Drei-Schichten-Architektur**:

```text
[ React 19 Client (Port 5173) ] -> HTTP / JSON (Axios + JWT) -> [ Express.js Backend (Port 5000) ] -> SQLite Database (better-sqlite3)
```

- **Frontend:** React 19 (SPA) gebündelt mit Vite, clientseitiges Routing via React Router v7.
- **Backend:** Node.js mit Express.js als RESTful API. Absicherung der Endpunkte über Auth-Middleware (JWT).
- **Datenbank:** SQLite (_better-sqlite3_) zur relationalen Speicherung von Benutzerdaten (_users_ Table)

---

## Setup-Anleitung (Inbetriebnahme in unter 10 Minuten)

Die Applikation lässt sich reproduzierbar mittels Docker-Compose oder manuell im Entwicklungsmodus ausführen.

### Option A: Start via Docker-Compose

**Voraussetzung:** Docker Container Runtime und Docker-Compose müssen installiert sein.

1. Repository klonen:

```bash
git clone https://github.com/Guakocius/htwg.git
cd htwg/sem5/webapp
```

2. Multi-Container-Environment starten:

```bash
docker-compose up --build -d
```

3. Applikation im Browser öffnen:

- **Frontend:** http://localhost:5173
- **Backend API:** http://localhost:5000

### Option B: Manuelle lokale Installation (Entwicklungsmodus)

**Voraussetzung:** Node.js (v18+) und npm.

1. Backend starten:

```bash
npm install --legacy-peer-deps
npx tsx src/backend/server/server.ts
```

(Backend läuft auf http://localhost:5000)

2. Frontend starten (in neuem Terminal):

```bash
npm install --legacy-peer-deps
npm run dev
```

(Frontend läuft auf http://localhost:5173)

---

## Testuser / Zugangsdaten für die Bewertung

Für den sofortigen Test der Anwendung stehen folgende vordefinierte Accounts zur Verfügung:

| Rolle         | Benutzername | Passwort      | Beschreibung                                         |
| ------------- | ------------ | ------------- | ---------------------------------------------------- |
| Standard-User | testuser     | Password123!  | Für reguläre Login-Tests und neue Highscore-Versuche |
| Top-Player    | player1      | SecurePass123 | Existierender Account im globalen Leaderboard        |

Hinweis: Neue Accounts können jederzeit direkt über die _/register_-Seite in der Anwendung angelegt werden.

---

## Demo-Video

Ein 3- bis 5-minütiges Demonstrations-Video (Screencast) zeigt die Registrierung, den Login, die Game-Mechanik sowie die Aktualisierung des Leaderboards.

---

## Bekannte Einschränkungen & Offene Punkte

1. **Client-side Physics:** Die Kollisionserkennung läuft aktuell clientseitig im Browser. Bei extrem hoher Auslastung des Client-Systems kann es vereinzelt zu Schwankungen in den Frameraten kommen.
2. **Audio-Effekte:** Sounds für Sprünge und Kollisionen sind in der aktuellen Version noch nicht implementiert.
3. **Session Expiry Handling:** Läuft das JWT-Token ab, muss sich der Nutzer manuell neu einloggen; ein automatischer Refresh-Token-Handshake ist als zukünftige Ausbaustufe geplant.
