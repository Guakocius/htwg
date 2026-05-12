# Flappy Bird

**Team:** Guakocius
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

| Kriterium                         | Datei                                                           | Zeile / Hinweis                                                  |
| --------------------------------- | --------------------------------------------------------------- | ---------------------------------------------------------------- |
| npm + Vite                        | package.json, vite.config.ts                                    | Projekt-Root                                                     |
| TypeScript aktiv genutzt          | register.html, login.html                                       | Z. 14-26                                                         |
| Responsives Layout (Flexbox/Grid) | style.css                                                       | Z. 8,40,46,69-70,128,141-142,163                                 |
| Media Query                       | styles.css                                                      | Z. 139-157                                                       |
| URL-Struktur                      | index.html, register.html login.html game.html leaderboard.html | Pfade: /, /src/register, /src/login, /src/game, /src/leaderboard |
