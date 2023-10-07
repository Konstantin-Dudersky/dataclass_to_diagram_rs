docker run -p8000:8000 -d yuzutech/kroki

## C4

https://github.com/plantuml-stdlib/C4-PlantUML

## Генерировать перечисления для domain/sprite

Клонировать репозиторий с библиотекой:

```bash
git clone https://github.com/tupadr3/plantuml-icon-font-sprites.git
```

Перейти в папку с файлами, например devicons2:

```bash
cd plantuml-icon-font-sprites/devicons2/
```

Создать файл \_generate.py:

```py
import os

OUT_FILENAME = "_enum.txt"
SCRIPT_FILENAME = "_generate.py"
sprites: set[str] = set()
for _, _, filenames in os.walk(os.getcwd()):
    for filename in filenames:
        if filename in [OUT_FILENAME, SCRIPT_FILENAME]:
            continue
        sprite = filename.split(".")[0]
        if sprite[0].isdigit():
            sprite = "_" + sprite
        if sprite == "box":
            sprite = "_box"
        sprites.add(sprite)

with open(OUT_FILENAME, "w", encoding="utf-8") as stream:
    for sprite in sorted(sprites):
        line = f'    {sprite},\n'
        stream.write(line)
```

Запустить на выполнение:

```bash
python3 _generate.py
```

Содержимое файла \_enum.txt вставить в перечиление
