# Flappy Bird

**Team:** Guakocius<br>
**Repository:** [https://github.com/Guakocius/htwg/tree/main/sem5/webapp](https://github.com/Guakocius/htwg/tree/main/sem5/webapp)

## 1. Architekturskizze & Technologie-Entscheidung

| Stack              | Komponente                                                                              |
| ------------------ | --------------------------------------------------------------------------------------- |
| Frontend           | Single Page Application (React + TS), Routing via React Router, HTTP Requests via Axios |
| HTTP REST-Anfragen | POST/GET                                                                                |
| Backend            | REST API (Express + TS), CORS & JSON Middleware, JWT-Authentifizierung & Bcrypt-Hashing |
| Mongoose           | ODM Connection                                                                          |
| MongoDB            | Datenbank, persistente Speicherung von Usern                                            |

### Begründung zur Systemarchitektur (SPA vs. SSR)

Für dieses Projekt wurde bewusst eine klassische Client-Side-Rendered Single Page Application
gewählt und auf Server-Side Rendering (SSR wie z.B. Next.js) oder Static Site Generation verzichtet.
Da es sich hierbei um ein interaktives Arcade-Spiel, namentlich Flappy Bird, mit hochdynamischen
Zuständen, Benutzer-Authentifizierung und Echtzeit-Leaderboards handelt, bringt SSR keinen
signifikanten Mehrwert. SEO-Optimierung ist für eine passwortgeschützte Spieleanwendung nicht
relevant. Eine SPA bietet zudem durch das asynchrone Nachladen von Daten via REST-API eine
flüssigere Benutzererfahrung ohne störende Seiten-Reloads.

---

## 2. Testuser-Credentials

Für die Bewertung und das Testing können folgende fiktive Zugangsdaten verwendet werden:

- **Username:** `TestUser`
- **Password:** `testpassword123!`

---

## 3. Kriterien-Zuordnung

| Kriterium                   | Beschreibung                                       | Datei / Code-Stelle                                                           |
| :-------------------------- | :------------------------------------------------- | :---------------------------------------------------------------------------- |
| **React-Router**            | 5 Routen definiert und `<Routes>` genutzt          | `src/App.tsx`                                                                 |
| **Navigation**              | Programmatische Navigation via `useNavigate`       | `src/componnents/Navbar.tsx`, `src/pages/Login.tsx`, `src/pages/Register.tsx` |
| **Datenfetching und REST**  | `POST`-Methode via Axios realisiert                | `src/components/LoginForm.tsx` (in `handleSubmit`)                            |
| **Fehler- und Ladezustand** | Clientseitige Validierung und API-Catch-Blöcke     | `src/components/LoginForm.tsx` (`validate()`, `catch ()`)                     |
| **Geteilter State**         | Mode-Toggling (Light/Dark Theme) via DOM / Session | `src/components/Navbar.tsx`                                                   |
| **Tests**                   | 3 funktionale Kernlogik-Tests geschrieben          | `tests/LoginForm.test.tsx`                                                    |
| **Backend-Server**          | Node.js-Server mit TypeScript und Express-API      | `src/backend/server/server.ts`                                                |
| **Datenbank**               | Persistente Datenhaltung via MongoDB und Mongoose  | `src/backend/server/server.ts`                                                |
| **Authentifizierung**       | Passwort-Hashing, Registrierung, JWT-Login         | `src/backend/server/server.ts` (`/register`, `/login`)                        |

---

## 4. Projekt starten

### Backend starten:

```bash
npm install --legacy-peer-deps
npm run start
```

### Frontend starten:

```bash
npm install --legacy-peer-deps
npm run dev
```

### Tests ausführen:

```bash
npm test
```
