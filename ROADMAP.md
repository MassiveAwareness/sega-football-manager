# 📅 Sega Football Manager - Fejlesztési Roadmap

Ez a dokumentum a játék fejlesztési lépéseit tartalmazza fázisokra bontva
Cél a Sega Genesis korszak (Premier Manager, FIFA Soccer 95) hangulatának megőrzése modern Rust alapokon

## 🛠️ Fázis 1: Alapok stabilizálása és UI csiszolás (Jelenlegi fázis)
A cél: A jelenlegi kód optimalizálása és a navigáció logikusabbá tétele.

- [x] **Projekt modularizálása:** Kód szétbontása (`main`, `models`, `ui`, `constants`)
- [x] **Sega Stílus:** Kék/Sárga/Fehér színvilág, keretek, Quaroxe font
- [x] **Alap szimuláció:** Csapaterősség alapú eredmények, reálisabb gólszámok (Poisson-eloszláson alapuló random generálás)
- [ ] **Szezongenerálás:** Véletlenszerű párosítás helyett "Round Robin" (mindenki játszik mindenkivel oda-vissza) menetrend generálása a szezon elején
- [ ] **UI Navigáció:** Egér támogatás hozzáadása a gombokhoz (a billentyűzet mellé), vagy egy kijelölés (highlight) rendszer a menüpontokhoz

## ⚽ Fázis 2: A Játékosok bevezetése
A cél: A csapat ne csak egy szám (`strength`) legyen, hanem játékosok összessége, ez a manager játék lelke

- [ ] **Player struktúra:**
    ```rust
    struct Player {
        name: String,       // szöveg
        position: Position, // poszt (külön enum)
        skill: u8,          // 8-bites egész szám, 1-100 között
        age: u8,            // 8-bites egész szám, 15-45 között
        value: u32          // 32-bites egész szám
    }
    ```
- [ ] **Csapat generátor:** Véletlenszerű nevek (előre megadott nevek alapján), illetve statisztikák generálása indításkor
- [ ] **Kezdőcsapat képernyő:** Egy új menüpont, ahol kilistázható a teljes keret, és látszik, ki kezdő (11 fő)
- [ ] **Szimuláció update:** A csapat erejét a *kezdőcsapatban* lévő játékosok átlaga adja ki, nem egy statikus szám

## 📢 Fázis 3: Meccs Motor 2.0 (Szöveges kommentár)
A cél: A meccs ne csak egy képernyő mögé rejtett, egyetlen szimulált pillanat legyen; hanem legyen izgalma is

- [ ] **Szöveges közvetítés:** A "MatchSimulation állapotban fusson egy időzítő (0-90 perc)
- [ ] **Események:**
    - "12' - Nagy helyzet az ETO előtt..."
    - "13' - GÓÓÓL! Kovács helyezi a labdát a jobb alsó sarokba..."
    - "14' - Sárga lap!!!"
- [ ] **Látvány:** Egy csík (progress bar) mutatja majd az időt
- [ ] **Interaktivitás:** - Lehet majd "gyorsítani", illetve át is ugrani a meccset

## 💰 Fázis 4: Menedzsment és Gazdaság
A cél: Legyen tétje a játéknak

- [ ] **Pénzügyek:** Költségvetés bevezetése. Győzelem = pénz + jegybevétel
- [ ] **Átigazolási piac:**
    - Listázhatóak az elérhető játékosok
    - "Vétel" gomb: Pénz levonása, játékos átkerülése a `teams[player_index]`-be
- [ ] **Fizetések:** A játékosoknak legyen heti bérük, amit le kell vonni

## 💾 Fázis 5: Perszisztencia (Mentés / Betöltés)
A cél: Ne vesszen el a haladás kilépéskor

- [ ] **Serde integráció:** A `serde` és `serde_json` csomagok hozzáadása
- [ ] **Mentési rendszer:** A `Game` struktúra szerializálása JSON fájlba (`progress.json`)
- [ ] **Betöltési rendszer:** Indításkor (amennyiben a fájl létezik) betölti az állapotot

## 🏆 Fázis 6: Hosszú távú célok
A cél: Végtelen játékmenet

- [ ] **Szezon vége:** Bajnokavatás, kiesés (amennyiben van alsóbb osztály), feljutás
- [ ] **Öregedés:** A játékosok öregednek, visszavonulnak, helyükre jönnek fiatalok (regen)
- [ ] **Kupa:** Külön hazai kupasorozat ág a bajnokságok mellett
