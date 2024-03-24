use std::{
    env::current_dir,
    fs::{remove_dir, remove_dir_all, remove_file, rename},
    path::Path,
    process::exit,
};

use clap::Parser;
use walkdir::WalkDir;

/// Extrae todos los archivos multimedia (mp4, mpeg, mpg, avi, mkv, srt, vtt, ass, ssa) de un directorio (y subdirectorios) con un único comando y borra automáticamente** los archivos y carpetas no necesarios.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Argumentos {
    /// Extensiones válidas separadas por comas
    #[arg(short, long, default_value = "mp4,mpeg,mpg,avi,mkv,srt,vtt,ass,ssa")]
    extensiones: String,

    /// Aviso si el número de archivos que se van a borrar es superior al permitido
    #[arg(short, long, default_value_t = 15)]
    aviso: u32,
}

struct Configuracion {
    aviso: u32,
    extensiones_validas: Vec<String>,
}
struct Manejador {
    configuracion: Configuracion,
    rutas: Option<Vec<String>>,
    archivos_mover: Option<Vec<String>>,
    archivos_borrar: Option<Vec<String>>,
    directorios_borrar: Option<Vec<String>>,
}

impl Manejador {
    fn new(configuracion: Configuracion) -> Manejador {
        Manejador {
            configuracion,
            rutas: None,
            archivos_mover: None,
            archivos_borrar: None,
            directorios_borrar: None,
        }
    }
    fn recorrer_archivos(&mut self) {
        let mut rutas_encontradas = Vec::new();
        let directorio_ejecucion = current_dir().expect("no se ha encontrado current_dir");
        for v in WalkDir::new(directorio_ejecucion.clone()) {
            match v {
                Err(error) => {
                    eprintln!("ATENCIÓN: error recorriendo archivos {}", error);
                    continue;
                }
                Ok(archivo) => {
                    let ruta = archivo.path().display().to_string();
                    if directorio_ejecucion.display().to_string() == ruta {
                        continue;
                    }
                    rutas_encontradas.push(ruta);
                }
            }
        }
        if !rutas_encontradas.is_empty() {
            self.rutas = Some(rutas_encontradas)
        }
    }
    fn cribar_archivos(&mut self) {
        if self.rutas.is_none() {
            return;
        }

        let mut archivos_mover = Vec::new();
        let mut archivos_borrar = Vec::new();
        let mut directorios_borrar = Vec::new();

        let rutas_encontradas = self.rutas.clone().unwrap();
        for v in rutas_encontradas {
            let ruta = Path::new(&v);
            if ruta.is_dir() {
                directorios_borrar.push(v);
                continue;
            }
            match ruta.extension() {
                None => archivos_borrar.push(v),
                Some(extension) => {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if self
                        .configuracion
                        .extensiones_validas
                        .contains(&String::from(ext))
                    {
                        archivos_mover.push(v)
                    } else {
                        archivos_borrar.push(v)
                    }
                }
            }
        }

        if !archivos_mover.is_empty() {
            self.archivos_mover = Some(archivos_mover)
        }
        if !archivos_borrar.is_empty() {
            self.archivos_borrar = Some(archivos_borrar)
        }
        if !directorios_borrar.is_empty() {
            self.directorios_borrar = Some(directorios_borrar)
        }
    }
    fn comprobar_archivos(&self) {
        match &self.archivos_borrar {
            None => return,
            Some(v) => {
                if v.len() < self.configuracion.aviso as usize {
                    return;
                }
                println!("ATENCIÓN: se van a borrar un número considerable de archivos: {}. En total son {} archivos.", v.join(", "), v.len());
                println!("¿Quieres continuar el proceso? [s/N]");
                let mut entrada_usuario = String::new();
                std::io::stdin()
                    .read_line(&mut entrada_usuario)
                    .expect("error leyendo la entrada de usuario");
                let entrada_usuario_min = entrada_usuario.to_lowercase();
                if entrada_usuario_min.trim() != "s" {
                    println!("PROCESO CANCELADO POR EL USUARIO");
                    exit(0);
                }
            }
        }
    }
    fn mover_archivos(&self) {
        match &self.archivos_mover {
            None => (),
            Some(v) => {
                for ruta in v {
                    let ruta_archivo = Path::new(&ruta);
                    let nombre_archivo = ruta_archivo
                        .file_name()
                        .expect("no se ha podido obtener el nombre del archivo");

                    let directorio_base = Path::new(".");
                    let nueva_ruta = directorio_base.join(nombre_archivo);
                    rename(ruta_archivo, nueva_ruta).expect("error moviendo archivo");
                }
            }
        }
    }
    fn borrar_archivos(&self) {
        match &self.archivos_borrar {
            None => (),
            Some(v) => {
                for ruta in v {
                    remove_file(ruta).expect("error borrando archivo");
                }
            }
        }
    }
    fn borrar_directorios(&self) {
        match &self.directorios_borrar {
            None => (),
            Some(v) => {
                for ruta in v {
                    // Aunque se han vaciado los archivos, pueden existir carpetas dentro de las carpetas
                    // Esto implicaría que remove_dir ocasione un error porque no está vacía
                    // En ese caso intentar un borrado completo, que lógicamente ocasionará error al intentar luego borrar esas carpetas
                    match remove_dir(ruta) {
                        Ok(_) => (),
                        Err(_) => {
                            let _ = remove_dir_all(ruta);
                        }
                    }
                }
            }
        }
    }
}

fn configurar_manejador(argumentos: Argumentos) -> Configuracion {
    let mut extensiones_validas = Vec::new();
    let extensiones = argumentos.extensiones.split(",");
    for v in extensiones {
        let minusculas = v.to_lowercase();
        let saneado = minusculas.trim().trim_start_matches(".");
        extensiones_validas.push(String::from(saneado))
    }
    if extensiones_validas.is_empty() {
        panic!("no se han encontrado extensiones válidas")
    }

    Configuracion {
        extensiones_validas,
        aviso: argumentos.aviso,
    }
}

fn main() {
    let mut manejador = Manejador::new(configurar_manejador(Argumentos::parse()));
    manejador.recorrer_archivos();
    manejador.cribar_archivos();
    manejador.comprobar_archivos();
    manejador.mover_archivos();
    manejador.borrar_archivos();
    manejador.borrar_directorios();
}
