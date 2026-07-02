# Flappy Bird

**Team:** Guakocius<br>
**Repository:** [https://github.com/Guakocius/htwg/tree/main/sem5/webapp](https://github.com/Guakocius/htwg/tree/main/sem5/webapp)

## Setup

```bash
npm install
npm run dev
```

oder einfach

```bash
./scripts/check-deps.sh
```

## Kriterien-Zuordnung M1

| Kriterium                 | Datei                                      | Zeile / Hinweis                                    |
| ------------------------- | ------------------------------------------ | -------------------------------------------------- |
| npm + Vite                | package.json, vite.config.ts               | Projekt-Root                                       |
| TypeScript aktiv genutzt  | src/types.ts, src/components/LoginForm.tsx | Z. 1-3 (eigene Typen), Z. 25-41 (Funktionen)       |
| Komponentenzerlegung      | src/components/                            | LeaderboardTable, LoginForm, Navbar                |
| Props-Übergabe            | src/components/LoginForm.tsx               | Z. 35-37                                           |
| useState                  | src/components/LoginForm.tsx               | Z. 7-10, Z. 22,23 (LoginForm-States)               |
| useEffect                 | src/components/Navbar.tsx                  | Z. 13-15 (zu anderer Seite navigieren)             |
| Durchgängige Nutzeraktion | src/components/LoginForm.tsx               | Formular zur Registrierung und Login, je nach Prop |
