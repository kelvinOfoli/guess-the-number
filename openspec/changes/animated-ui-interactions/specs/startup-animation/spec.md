## ADDED Requirements

### Requirement: Animated title banner on launch
The game SHALL display an ASCII-art title banner that reveals character by character when the program starts, before any game input is accepted.

#### Scenario: Title types in on startup
- **WHEN** the program is launched
- **THEN** the ASCII-art title "GUESS THE NUMBER" appears letter by letter with a short delay between each character

#### Scenario: Animation completes before game begins
- **WHEN** the startup animation is running
- **THEN** the game SHALL NOT prompt for user input until the full animation sequence has completed

### Requirement: Countdown before game start
The game SHALL display a 3-2-1 countdown animation after the title banner before presenting the first guess prompt.

#### Scenario: Countdown displays each number
- **WHEN** the title banner animation finishes
- **THEN** the numbers 3, 2, 1 SHALL appear in sequence, each on their own line, with a visible pause between them (at least 300ms per step)

#### Scenario: Game starts after countdown
- **WHEN** the countdown reaches 1 and its display duration elapses
- **THEN** the game SHALL immediately display the first guess prompt with no additional delay

### Requirement: Intro flavor text reveal
The game SHALL display a brief description line ("I'm thinking of a number between 1 and 100") using a typing effect after the countdown.

#### Scenario: Flavor text types out
- **WHEN** the countdown completes
- **THEN** the intro flavor text SHALL appear character by character at a rate of 20-40ms per character
