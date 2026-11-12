# Sudoku Solver – Simulated Annealing (Symulowane Wyżarzanie)

Projekt rozwiązujący łamigłówki **Sudoku 9x9** przy użyciu metaheurystyki **symulowanego wyżarzania** (*Simulated Annealing*), zaimplementowany w języku **Rust**.

Domyślnie program rozwiązuje jedno z najtrudniejszych znanych Sudoku na świecie – ułożone przez fińskiego matematyka Arto Inkalę w 2010 roku (*"World's Hardest Sudoku"*).

---

## 📋 Spis treści

- [Zasada działania algorytmu](#-zasada-działania-algorytmu)
- [Struktura projektu](#-struktura-projektu)
- [Wymagania](#-wymagania)
- [Instrukcja uruchomienia](#-instrukcja-uruchomienia)
- [Wizualizacja w terminalu](#-wizualizacja-w-terminalu)

---

## 🧠 Zasada działania algorytmu

Algorytm symulowanego wyżarzania modeluje proces stygnięcia kryształów w celu znalezienia globalnego minimum funkcji energii:

1. **Inicjalizacja (wypełnienie bloków $3 \times 3$):**
   - Pola początkowe (wskazówki) są oznaczane jako zablokowane (*fixed*).
   - Każdy z 9 podkwadratów $3 \times 3$ jest dopełniany brakującymi cyframi od 1 do 9 bez powtórzeń.
   - **Niezmiennik:** Każdy blok $3 \times 3$ zawsze zawiera permutację cyfr 1–9, więc konflikty mogą występować wyłącznie w wierszach i kolumnach.

2. **Funkcja energii ($E$):**
   - Energia to suma powtórzeń cyfr w każdym wierszu oraz w każdej kolumnie planszy.
   - Rozwiązanie jest znalezione w momencie, gdy **$E = 0$** (brak jakichkolwiek konfliktów).

3. **Przejście do stanu sąsiedniego:**
   - Losowo wybierany jest blok $3 \times 3$, a w nim dwa odblokowane (modyfikowalne) pola.
   - Wartości tych pól są zamieniane, po czym obliczana jest zmiana energii $\Delta E$.

4. **Kryterium akceptacji (kryterium Metropolisa):**
   - Jeżeli $\Delta E \le 0$ (nowy stan jest lepszy lub równy), zmiana jest natychmiast akceptowana.
   - Jeżeli $\Delta E > 0$ (nowy stan jest gorszy), zmiana zostaje zaakceptowana z prawdopodobieństwem:
     $$P = \exp\left(-\frac{\Delta E}{T}\right)$$
     gdzie $T$ to aktualna temperatura układu. Pozwala to na ucieczkę z minimów lokalnych.

5. **Chłodzenie i restarty:**
   - Temperatura początkowa: $T_0 = 0.5$
   - Współczynnik chłodzenia: $T \leftarrow T \cdot 0.99995$
   - W przypadku braku zbieżności w ramach $100\,000$ iteracji następuje automatyczny restart z nową permutacją początkową (do 200 prób).

---

## 📁 Struktura projektu

```text
├── Cargo.toml          # Konfiguracja projektu i zależności (rand)
├── Cargo.lock          # Zablokowane wersje bibliotek
├── .gitignore          # Reguły ignorowania plików w repozytorium Git
├── README.md           # Dokumentacja projektu
└── src/
    ├── main.rs          # Punkt wejściowy, definicja zadania, obsługa flag CLI
    ├── sudoku_solver.rs # Implementacja algorytmu wyżarzania i funkcji kosztu
    └── display.rs       # Wizualizacja planszy w terminalu z kodami kolorów ANSI
```

---

## 🛠 Wymagania

- [Rust](https://www.rust-lang.org/) (edycja 2024 / wersja 1.85+)
- Cargo (menedżer pakietów Rusta)
- Terminal ze wsparciem kodów ucieczki ANSI (dla animowanej wizualizacji)

---

## 🚀 Instrukcja uruchomienia

### 1. Tryb z interaktywną wizualizacją (domyślny)

Wyświetla na żywo animowaną planszę w terminalu, aktualną energię, temperaturę, liczbę iteracji oraz historię restartów:

```bash
cargo run --release
```

> **Wskazówka:** Flaga `--release` jest zalecana ze względu na wysoką wydajność obliczeń symulacji w zoptymalizowanym kodzie maszynowym.

### 2. Tryb headless (bez animacji ekranu)

Przydatny do szybkiego profilowania lub uruchamiania w środowiskach bez pełnego wsparciu dla terminala interaktywnego:

```bash
cargo run --release -- --no-display
```

Przykładowe wyjście w trybie headless:
```text
Running solver in headless mode…
✓ Solved!  Energy=0  Time=2.14s
8 1 2 | 7 5 3 | 6 4 9
9 4 3 | 6 8 2 | 1 7 5
6 7 5 | 4 9 1 | 2 8 3
------+-------+------
1 5 4 | 2 3 7 | 8 9 6
3 6 9 | 8 4 5 | 7 2 1
2 8 7 | 1 6 9 | 5 3 4
------+-------+------
5 2 1 | 9 7 4 | 3 6 8
4 3 8 | 5 2 6 | 9 1 7
7 9 6 | 3 1 8 | 4 5 2
```

---

## 🎨 Wizualizacja w terminalu

Interfejs konsolowy koloruje elementy planszy:
- **Pogrubione białe:** Początkowe cyfry planszy (niezmienne).
- **Zielone:** Cyfry dobrane przez algorytm, które nie generują konfliktów w wierszu/kolumnie.
- **Czerwone:** Cyfry powodujące konflikt (powtórzenie w wierszu lub kolumnie).
