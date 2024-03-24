# AUTOEXTRACTOR MULTIMEDIA
Extrae todos los archivos multimedia (mp4, mpeg, mpg, avi, mkv, srt, vtt, ass, ssa) de un directorio (y subdirectorios) con un único comando y **borra automáticamente** los archivos y carpetas no necesarios.

## Instalación y uso
1. Descargar el archivo binario desde la sección "Releases" del repositorio.
1. Colocar en un directorio que forme parte del $PATH
1. Desde la terminal, situarse en el directorio raíz que contenga **únicamente las carpetas con los archivos a recuperar y borrar**
```bash
cd /home/user/Descargas/Videos
```
4. Ejecutar el auto extractor desde el directorio de trabajo actual:
```bash
cd /home/user/Descargas/Videos
./autoextractor_multimedia
```
Después de la ejecución en el directorio de trabajo actual únicamente quedarán los archivos de las extensiones permitidas.

### Importante
La idea es que el procesado se realice automáticamente sin ningún tipo de interacción, no obstante, como existen operaciones de borrado de archivos existe una pequeña comprobación con relación al número de archivos que se van a borrar (imagina el problema que ocasionaría ejecutar este programa desde el directorio de trabajo /home/user, por ejemplo) 

Para evitar este problema, por defecto, si el **número de archivos a borrar es mayor que 15** aparecerá un aviso que exige la interacción del usuario. En el caso de que se esté seguro sobre el funcionamiento y quiera evitarse este aviso puede elevarse el número con el argumento --aviso=X (donde X es un u32, por lo que el número máximo sería: 4294967295)

### Uso
```bash
Usage: autoextractor_multimedia [OPTIONS]

Options:
  -e, --extensiones <EXTENSIONES>  Extensiones válidas separadas por comas [default: mp4,mpeg,mpg,avi,mkv,srt,vtt,ass,ssa]
  -a, --aviso <AVISO>              Aviso si el número de archivos que se van a borrar es superior al permitido [default: 15]
  -h, --help                       Print help
  -V, --version                    Print version
```
 ![Imgaen](https://i.imgur.com/tVeK1k2.png)
