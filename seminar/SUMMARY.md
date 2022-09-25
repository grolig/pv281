# 2021 Seminar summary

## Week 03
Structs, enums, CLI, files
### Demo
Person struct, Contact enum
    
## Week 04
Structs, enums, pattern matching, CLI, files
### Demo
Vytvořte program cat, který vypíše soubor na standardní výstup.
V tuto chvíli neřešte velikost souboru a předpokládejte,
že půjde pouze o malé soubory.
Diskutujte řešení pro velké soubory.
### Iteration
Pro získání 1 bodu za seminář vytvořte během následující hodiny program more.
More bude vypisovat obsah souboru po n řádcích.
Po stisknutí enteru vypíše dalších n řádků.
Implementaci proveďte tak,
aby fungovala i velkých souborech a zlikvidovala dostupnou paměť.

Použití:

`more --lines 40 --file input.txt`

`more -l 40 -f input.txt`
### Bonus
Vytvořte aplikaci formater,
který bude načítat csv a z něj vypisovat v terminálu ohraničenou tabulku.
Jednotlivé sloupce budou formátované na určitou délku a na určitou stranu.
To je možné ovlivnit parametry příkazové řádky.
V případě automatické detekce délky slupce přednačtěte 100 řádků a zjistěte délku.
Pokud dále narazíte na delší sloupec tak řádek zkraťte.

## Week 05
Vectors, iterators, hashmap, hashset, heap

### Demo
Vytvořte program fungující jako cache.
Vyberte vhodnou datovou strukturu pro provedení cache.
Operaci, kterou chcete kešovat, simulujte načtením ze standardního vstupu.
Navrhněte vhodné API pro práci.

### Iteration
Každá šalina má priradenú nejakú pravidelnú trasu - linku, po ktorej denne jazdí. 
Vašou úlohou je napísať krátky program, ktorý umožňuje nasledovné:

- vytvorenie linky, ktorá má číslo a zastávky
- vytvorenie zastávky, ktorá má názov, harmonogram (číslo linky, čas odchodu) a informáciu o tom, či má daná zastávka priamy prístup na vlakovú stanicu alebo nie
- výpis všetkých zastávok danej linky (zaujímajú nás iba názvy zastávok)
- výpis informácie, či je daná zastávka na danej linke
- pre zastávku a linku výpis nasledujúcej zastávky
- pre dve dané linky výpis všetkých spoločných zastávok (inak povedané, na ktorých zastávkach môžem prestúpiť z linky A na linku B)
- pre danú linku výpis zastávok s prístupom na vlakovú stanicu
- (optional) pre danú zastávku vypísať harmonogram, kde výpis jednej položky harmonogramu vyzerá napr.: Linka 3 - 14:10; harmonogram by sa mal vypísať v chronologickom poradí, pokiaľ majú dve linky rovnaký čas odchodu, tak na ich vzájomnom poradí nezáleží.

POZNÁMKA: Nepoužívajte cykly, pokiaľ to nie je nutné - preferované je použiť iterátory.

Pre inšpiráciu môžu poslúžiť linky v Brne: [Přehled linek | Jízdní řády Brno (jrbrno.cz)](http://www.jrbrno.cz/)

### Bonus
Vytvořte pseudo databázi, která je reprezentovaná vektorem.

Můžete do ní vkládat data nebo je odstraňovat.
Odstranění vyřešte filtrací a vrácením nového vektoru.

Hledání vyřešte pomocí indexu (vyberte pro něj vhodnou strukturu)
a vyhledávejte s jeho pomocí.

## Week 06
Generika, Smart Pointer, Moduly, Crates, Testování, Dokumentace

### Demo
Vytvořte aplikaci výstřižky,
která umožní uložení názvů článků, anotací a odkazů na články.

Články můžete otagovat.
Oddělte datové struktury a funkce do knihovny,
kterou použijete v aplikaci.

Funkce zdokumentujte a otestujte.

### Iteration
Vytvořte knihovnu pro fitness aplikace,
která poskytuje informace o uživateli.

Aplikace počítá BMI s leduje jeho historii,
dále ukládá historie srdečního tepu, měření tlaku aj.

Informace je možné vypsat za předchozí týden se zobrazením rozdílu oproti minulému dni.
Dále je možné zobrazit rozdíl před měsícem a před rokem.

Pro knihovnu vytvořte testy a dokumentaci.

### Bonus
Vytvořte konzolovou variantu hledače min.

Pro hru vytvořte vhodnou strukturu,
která obsahuje historii her (úspěšná/neúspěšná, počet tahů na dohrání),
informace o hře (počet tahů, a pole), a pole s minami.

Tah načtěte od hráče jako pozici x,y,typ tahu (označení miny nebo šlápnutí).
Po každém tahu vypište herní pole se skrytými minami.
Při špatném tahu ukažte, kde byly miny a ukončete hru.

## Week 07,
Lifetimes, Plánování procesů, Paralelismus v Rustu

### Demo
Vytvořte knihovny pro konkurentní slovník.
Slovník je přístupný všem pro čtení,
ale v případě zápisu je přístupný pouze jednomu zapisujícímu.

### Iteration
Vytvořte vícevláknovou aplikaci večeřící filozofové.

V aplikaci je n filozofů a n hůlek.
Filozof je reprezentovaný jedním vláknem.
Hůlka představuje sdílený zdroj.
Každý filozof přemýšlí, jí nebo čeká na hůlku.

Čas každé operace je náhodný mezi 500ms až 5000ms.
Reprezentujte jej uspáním threadu.
### Bonus
Předpokládejme, že cigareta vyžaduje tři ingredience, aby mohla být vykouřena:
- tabák
- papír
- zápalky

Dále předpokládejme, že kolem stolu jsou tři kuřáci,
z nichž každý má nekonečné zásoby jedné ze tří ingrediencí –
jeden kuřák má nekonečné zásoby tabáku,
další má nekonečné zásoby papíru
a třetí má nekonečné zásoby zápalek.

Předpokládejme také, že je zde rozhodčí nekuřák.
Rozhodčí umožňuje kuřákovi vyrobit si cigaretu tak,
že si vybere naslepo (nedeterministicky) dva kuřáky,
vezme jednu položku z jejich zdrojů a umístí tyto položky na stůl.
Následně upozorní třetího kuřáka, že provedl tuto akci.
Třetí kuřák vezme věci ze stolu
a použije je (spolu se svým vlastním zdrojem) pro výrobu cigarety,
kterou bude následně chvíli kouřit.
Mezitím rozhodčí rozhodne, že stůl je opět prázdný,
znovu náhodně vybere dva kuřáky a umístí jejich věci na stůl.
Tento proces pokračuje donekonečna.

Kuřáci si neshromažďují věci ze stolu;
kuřák si začne opět balit další cigaretu,
pouze pokud již dokouřil poslední.
Pokud by rozhodčí položil tabák a papír na stůl,
zatímco muž se zápalkami kouří, tabák a papír zůstanou nedotčeny na stole,
dokud tento kuřák nedokouří svou cigaretu a nesebere je.

## Week 08
### Demo
### Iteration
### Bonus

## Week 09
### Demo
### Iteration
### Bonus

## Week 10
### Demo
### Iteration
### Bonus

## Week 11
### Demo
### Iteration
### Bonus

## Week 12
### Demo
### Iteration
### Bonus

## Week 13
### Demo
### Iteration
### Bonus
