use std::process::Command;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create the build directories we need
    std::fs::create_dir_all("build")?;
    std::fs::create_dir_all("build/kernel")?;

    std::fs::create_dir_all("build/isofiles")?;
    std::fs::create_dir_all("build/isofiles/boot")?;
    std::fs::create_dir_all("build/isofiles/boot/grub")?;

    std::fs::copy("config/grub.cfg", "build/isofiles/boot/grub/grub.cfg")?;

    // Construct the path to the build directory
    let build_path = Path::new("build").canonicalize()?;

    let kernel_arch_dir = 
        Path::new("kernel")
        .join("src")
        .join("arch")
        .join("x86_64")
        .canonicalize()?;

    let boot_assembly = 
        Path::new(&kernel_arch_dir)
        .join("boot.asm")
        .canonicalize()?;
    
    let boot64_assembly = 
        Path::new(&kernel_arch_dir)
        .join("boot64.asm")
        .canonicalize()?;

    println!("Assembling 'boot.asm'");
    Command::new("nasm")
        .current_dir(&build_path)
        .args(&[
            "-g",
            "-f", "elf64",
            boot_assembly.to_str().unwrap(),
            "-o", "boot.o"])
        .status()?.success();

    Command::new("nasm")
        .current_dir(&build_path)
        .args(&[
            "-g",
            "-f", "elf64",
            boot64_assembly.to_str().unwrap(),
            "-o", "boot64.o"])
        .status()?.success();

    println!("Building the kernel");

    let kernel_path = Path::new("kernel").canonicalize()?;
    let kernel_build_path = 
        Path::new(&build_path)
        .join("kernel")
        .canonicalize()?;

    Command::new("cargo")
        .current_dir(&kernel_path)
        .args(&[
            "build",
            "--target-dir", kernel_build_path.to_str().unwrap()])
        .status()?.success();

    let linker_path = 
        Path::new("kernel")
        .join("src")
        .join("arch")
        .join("x86_64")
        .join("linker.ld")
        .canonicalize()?;

    let kernel_lib_path = 
        Path::new(&kernel_build_path)
        .join("x86_64-kernel")
        .join("debug")
        .join("libkernel.a")
        .canonicalize()?;
    
    println!("Linking the final binary");
    Command::new("ld")
        .current_dir(&build_path)
        .args(&[
            "-n", 
            "-T", linker_path.to_str().unwrap(),
            "boot.o",
            "boot64.o",
            kernel_lib_path.to_str().unwrap(),
            "-o", "kernel.bin"])
        .status()?.success();

    println!("Copying the final binary");
    std::fs::copy("build/kernel.bin", "build/isofiles/boot/kernel.bin")?;

    println!("Creating the iso");
    Command::new("grub-mkrescue")
        .current_dir(&build_path)
        .args(&[
            "-o", "nanoos.iso",
            "isofiles"])
        .status()?.success();
                    
    Ok(())
}
