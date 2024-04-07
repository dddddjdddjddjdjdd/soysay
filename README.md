# soysay
![soysay showcase](https://i.ibb.co/DQL96fW/showcase.png)

make LE SOY BOYS say whatever you want!
Officially supports Python 3.8.0 and above. The script may work on older versions but Py2Exe only works on 3.8+ which leaves no real reason to bother testing it on 3.7 and below. I intend on retaining support for 3.8.x until at the very least October 2024 when Python drops support for 3.8.
## Usage
soysay [input] (must be in quotation marks) (args)
## Options
 --h, --help     Prints out this help message (also shown when no input is provided)
 
 --random        Picks a random soyjak to display
 
--soyjak        Uses the standard default soyjak  

--snoojak       Uses the Snoojak

--gapejak       Uses the Gapejak

--cobson        Uses the Cobson soyjak

--markiplier    Uses the Markiplier soyjak

--chud          Uses the Chudjak

Note: If no options are provided, soysay will use the standard soyjak

## Future plans
- Make it so you don't need to put your string in quotation marks
- Make arguments work properly so they don't have to be placed after the string

## Building
Requirements:
- Python 3.8.0 - 3.11.x
- Py2exe 0.13.0.1

Method:
1. Make sure soysay.py and setup.py are in the same directory
2. Type setup.py py2exe and hit enter
