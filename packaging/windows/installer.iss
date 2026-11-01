; CREW Client — Official Windows Setup Wizard Script
; Inno Setup 6.x

#define MyAppName "CREW Client"
#define MyAppVersion "2.1.0"
#define MyAppPublisher "CREW Network"
#define MyAppURL "https://client.crewnetwork.space/"
#define MyAppExeName "CREW-Client.exe"

[Setup]
AppId={{6E48A96C-98A6-4447-9759-CREWCLIENT210}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName={#MyAppName} v{#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={userappdata}\CREW Client
DisableDirPage=no
AppendDefaultDirName=yes
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=no
LicenseFile=..\..\LICENSE
OutputDir=..\..\dist
OutputBaseFilename=CREW-Client-Setup-x64
SetupIconFile=..\icons\crew.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
UninstallDisplayName={#MyAppName}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=lowest
DisableWelcomePage=no
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "turkish"; MessagesFile: "compiler:Languages\Turkish.isl"

[CustomMessages]
turkish.WelcomeLabel1=CREW Client Kurulum Sihirbazı
turkish.WelcomeLabel2=Bu sihirbaz, CREW Client v{#MyAppVersion} uygulamasını bilgisayarınıza kuracaktır.%n%nDevam etmeden önce diğer tüm uygulamaları kapatmanız önerilir.
turkish.FinishedHeadingLabel=CREW Client kurulumu tamamlandı!
turkish.FinishedLabel=CREW Client başarıyla kuruldu. Kurulumdan çıkmak için Son düğmesine tıklayın.
turkish.LaunchProgram=CREW Client uygulamasını şimdi başlat
turkish.CreateDesktopIcon=Masaüstü kısayolu oluştur

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\..\CREW-Client.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\icons\crew.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\crew.ico"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\crew.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Only delete temporary launcher logs/caches if any inside {app}, NEVER delete user saves or configs
Type: files; Name: "{app}\*.log"
