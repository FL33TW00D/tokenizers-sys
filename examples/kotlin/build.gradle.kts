plugins {
    kotlin("jvm") version "1.9.20"
    application
}

group = "com.tokenizers"
version = "1.0.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

application {
    mainClass.set("TokenizerExampleKt")
}

kotlin {
    jvmToolchain(21)
}

tasks.register<Exec>("compileJni") {
    description = "Compile JNI shared library"
    
    val javaHome = System.getProperty("java.home")
    val osName = System.getProperty("os.name").lowercase()
    val libExtension = when {
        osName.contains("mac") -> "dylib"
        osName.contains("linux") -> "so"
        osName.contains("windows") -> "dll"
        else -> "so"
    }
    
    val platformInclude = when {
        osName.contains("mac") -> "darwin"
        osName.contains("linux") -> "linux"
        osName.contains("windows") -> "win32"
        else -> "linux"
    }
    
    commandLine(
        "gcc",
        "-shared",
        "-fPIC",
        "-I$javaHome/include",
        "-I$javaHome/include/$platformInclude",
        "-I${project.projectDir}/../../bindings",
        "-L${project.projectDir}/../../target/release",
        "-ltokenizers_sys",
        "-o", "${project.projectDir}/libtokenizers_jni.$libExtension",
        "${project.projectDir}/../java/jni_wrapper.c"
    )
}

tasks.named<JavaExec>("run") {
    dependsOn("compileJni")
    systemProperty("java.library.path", project.projectDir.absolutePath)
}

tasks.named("compileKotlin") {
    dependsOn("compileJni")
}