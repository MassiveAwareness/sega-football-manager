# ⚽ Sega Football Manager

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![Engine](https://img.shields.io/badge/Engine-Macroquad-blue.svg)

Egy klasszikus, 90-es éveket idéző futball menedzser játék, amely tisztán **Rust** nyelven íródott a **Macroquad** játékmotor segítségével.

A projekt célja egy, a **Sega Genesis / Mega Drive** korszak (pl. *Premier Manager 97*, *FIFA Soccer 95*) hangulatának és esztétikájának megidézése modern technológiával, függőségi pokol (DLL linking) nélkül.

---

## 🎮 Funkciók

* **Retro UI Design:** Autentikus sötétkék/sárga színpaletta, vastag keretek és pixel art tipográfia
* **Bajnokság Szimuláció:** 8 csapatos magyar bajnokság (NB1 inspiráció)
* **Tabella Követés:** Dinamikus tabella, amely pontszám és gólkülönbség alapján rendezi a csapatokat
* **Realisztikus Eredmények:** A meccsmotor figyelembe veszi a csapatok pillanatnyi erősségét és a hazai pálya előnyét, elkerülve az irreális (pl. konstans 4:4) eredményeket
* **Modularizált Kód:** Tiszta, szétválasztott architektúra (`UI`, `Logika`, `Konstansok`)

## 🚀 Telepítés és Futtatás

A játék futtatásához csak a Rust keretrendszerre van szükség.

### 1. Előfeltételek
Győződj meg róla, hogy a Rust telepítve van a gépeden:
```bash
rustc --version
```

Ha nincs, [innen telepítheted](https://rustup.rs)!

### 2. Klónozás és Assets beállítása
Töltsd le a projektet:
```bash
git clone https://github.com/MassiveAwareness/sega_football_manager.git
cd sega_football_manager
```

A struktúrának így kell kinéznie:
```graphql
sega-football-manager/
|-- assets/
|   |-- fonts/
|   |   |-- font.ttf
|-- Cargo.toml
|-- src/
    |
    |--- ...
```

### 3. Futtatás
Afejlesztői környezetből (pl. VS Code) vagy terminálból:
```bash
cargo run
```

A fordítás után az ablak automatikusan megnyílik.

---

## 🕹️ Irányítás
A játékot kizárólag billentyűzettel lehet irányítani, hűen a régi konzolos stílushoz.

| **Billentyű** | **Funkció / Esemény** |
|-----------|-------------------|
| ENTER | Játék indítása a főmenüből |
| A | Következő hét |
| S | Tabella megtekintése |
| SPACE | Visszalépés / Tovább (függően a játékállapottól) |
| ESC | Kilépés a főmenübe |

---

## 🏗️ Projekt Felépítése
A kód négy fő modulra van bontva a könnyebb fejleszthetőség érdekében:

* `src/main.rs`: A belépési pont. Itt található a "Game Loop" és az állapotgép (`AppState`), ami váltogat a menük között.
* `src/models.rs`: A tiszta üzleti logika. Itt van definiálva a `Team` és a `Game` struktúra, valamint itt fut a meccsszimuláció algoritmusa. Egyáltalán nem tartalmaz grafikai kódot.
* `src/ui.rs`: A megjelenítésért felelős réteg. Segédfüggvények a dobozok (`draw_sega_box`) és a szövegek kirajzolásához.
* `src/constants.rs`: A globális beállítások és a Sega-színpaletta definíciói.

## 🧮 Szimulációs Logika
A jelenlegi verzió (v0.1.0) egy valószínűségszámításon alapuló modellt használ:

1 - Minden meccsen a csapatok **kb. 6 gólhelyzetet** (dobást) kapnak
2 - A gól esélyét a támadó csapat ereje (`strength`) határozza meg, módosítva a védő csapat erejével
3 - A hazai csapat +5 erősség bónuszban részesül
4 - Ez a rendszer biztosítja, hogy az erősebb csapatok gyakrabban nyernek, de a meglepetés és a döntetlen is realisztikus mértékben van benne a pakliban

---

## 🗺️ Fejlesztési Útvonal
A fejlesztés folyamatos, a következő tervezett lépések (lásd `ROADMAP.md` a rászletekért):

- [x] Alap rendszer és UI
- [ ] Játékosok (név, poszt, skill) implementációja
- [ ] Átigazolási piac
- [ ] Részletesebb meccsközvetítés (szöveges kommentárral)
- [ ] Mentés / Betöltés funkció

**© 2025 - MassiveAwareness**
