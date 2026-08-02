using System;
using System.Drawing;
using System.IO;
using System.IO.Compression;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace AntaiInstaller
{
    static class Program
    {
        [STAThread]
        static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new InstallerForm());
        }
    }

    public class InstallerForm : Form
    {
        private Label lblTitle;
        private Label lblSub;
        private Label lblPath;
        private TextBox txtPath;
        private CheckBox chkDesktop;
        private CheckBox chkStartMenu;
        private CheckBox chkLaunch;
        private ProgressBar progressBar;
        private Button btnInstall;
        private Label lblStatus;

        public InstallerForm()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            this.Text = "ANTAI Sentinel — Setup Installatore Nativo";
            this.Size = new Size(540, 420);
            this.StartPosition = FormStartPosition.CenterScreen;
            this.FormBorderStyle = FormBorderStyle.FixedSingle;
            this.MaximizeBox = false;
            this.BackColor = Color.FromArgb(5, 5, 8); // Obsidian Black
            this.ForeColor = Color.White;
            this.Font = new Font("Segoe UI", 9F, FontStyle.Regular);

            // Title Banner
            lblTitle = new Label
            {
                Text = "ANTAI SENTINEL",
                Font = new Font("Segoe UI", 16F, FontStyle.Bold),
                ForeColor = Color.FromArgb(255, 0, 60), // Imperial Crimson
                Location = new Point(24, 20),
                AutoSize = true
            };

            lblSub = new Label
            {
                Text = "Installatore Nativo — Sistema Immunitario AI per Sviluppatori",
                Font = new Font("Segoe UI", 9F, FontStyle.Regular),
                ForeColor = Color.FromArgb(160, 160, 175),
                Location = new Point(26, 52),
                AutoSize = true
            };

            // Install Path
            lblPath = new Label
            {
                Text = "Cartella di Destinazione:",
                Location = new Point(26, 95),
                AutoSize = true,
                ForeColor = Color.FromArgb(220, 220, 235)
            };

            string defaultPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "Programs", "ANTAI");

            txtPath = new TextBox
            {
                Text = defaultPath,
                Location = new Point(26, 118),
                Size = new Size(470, 25),
                BackColor = Color.FromArgb(18, 18, 26),
                ForeColor = Color.White,
                BorderStyle = BorderStyle.FixedSingle
            };

            // Checkboxes
            chkDesktop = new CheckBox
            {
                Text = "Crea collegamento sul Desktop",
                Checked = true,
                Location = new Point(26, 160),
                AutoSize = true,
                ForeColor = Color.FromArgb(200, 200, 220)
            };

            chkStartMenu = new CheckBox
            {
                Text = "Aggiungi al Menu Start di Windows",
                Checked = true,
                Location = new Point(26, 190),
                AutoSize = true,
                ForeColor = Color.FromArgb(200, 200, 220)
            };

            chkLaunch = new CheckBox
            {
                Text = "Avvia ANTAI Sentinel al termine dell'installazione",
                Checked = true,
                Location = new Point(26, 220),
                AutoSize = true,
                ForeColor = Color.FromArgb(200, 200, 220)
            };

            // Progress Bar
            progressBar = new ProgressBar
            {
                Location = new Point(26, 260),
                Size = new Size(470, 18),
                Style = ProgressBarStyle.Blocks,
                Visible = false
            };

            lblStatus = new Label
            {
                Text = "Pronto per l'installazione.",
                Location = new Point(26, 288),
                Size = new Size(470, 20),
                ForeColor = Color.FromArgb(0, 240, 255), // Cyan
                Font = new Font("Segoe UI", 8.5F, FontStyle.Italic)
            };

            // Install Button
            btnInstall = new Button
            {
                Text = "🚀 INSTALLA ANTAI SENTINEL",
                Location = new Point(26, 318),
                Size = new Size(470, 42),
                BackColor = Color.FromArgb(255, 0, 60),
                ForeColor = Color.White,
                FlatStyle = FlatStyle.Flat,
                Font = new Font("Segoe UI", 10F, FontStyle.Bold),
                Cursor = Cursors.Hand
            };
            btnInstall.FlatAppearance.BorderSize = 0;
            btnInstall.Click += BtnInstall_Click;

            this.Controls.Add(lblTitle);
            this.Controls.Add(lblSub);
            this.Controls.Add(lblPath);
            this.Controls.Add(txtPath);
            this.Controls.Add(chkDesktop);
            this.Controls.Add(chkStartMenu);
            this.Controls.Add(chkLaunch);
            this.Controls.Add(progressBar);
            this.Controls.Add(lblStatus);
            this.Controls.Add(btnInstall);
        }

        private async void BtnInstall_Click(object sender, EventArgs e)
        {
            string targetDir = txtPath.Text.Trim();
            if (string.IsNullOrEmpty(targetDir)) return;

            btnInstall.Enabled = false;
            progressBar.Visible = true;
            progressBar.Value = 10;
            lblStatus.Text = "Estrazione file di ANTAI in corso...";

            await Task.Run(() =>
            {
                try
                {
                    if (!Directory.Exists(targetDir))
                    {
                        Directory.CreateDirectory(targetDir);
                    }

                    // Extract embedded payload.zip resource
                    Assembly assembly = Assembly.GetExecutingAssembly();
                    using (Stream stream = assembly.GetManifestResourceStream("payload.zip"))
                    {
                        if (stream != null)
                        {
                            string tempZip = Path.Combine(Path.GetTempPath(), "antai_payload_temp.zip");
                            using (FileStream fs = File.Create(tempZip))
                            {
                                stream.CopyTo(fs);
                            }

                            using (ZipArchive archive = ZipFile.OpenRead(tempZip))
                            {
                                foreach (ZipArchiveEntry entry in archive.Entries)
                                {
                                    string destinationPath = Path.GetFullPath(Path.Combine(targetDir, entry.FullName));
                                    if (entry.FullName.EndsWith("/") || entry.FullName.EndsWith("\\"))
                                    {
                                        Directory.CreateDirectory(destinationPath);
                                    }
                                    else
                                    {
                                        Directory.CreateDirectory(Path.GetDirectoryName(destinationPath));
                                        entry.ExtractToFile(destinationPath, true);
                                    }
                                }
                            }

                            if (File.Exists(tempZip)) File.Delete(tempZip);
                        }
                    }

                    this.Invoke((MethodInvoker)delegate
                    {
                        progressBar.Value = 60;
                        lblStatus.Text = "Creazione collegamenti Desktop e Menu Start...";
                    });

                    string exePath = Path.Combine(targetDir, "ANTAI-Sentinel-Desktop.exe");
                    if (!File.Exists(exePath))
                    {
                        exePath = Path.Combine(targetDir, "start_antai.bat");
                    }

                    // Create Desktop Shortcut
                    if (chkDesktop.Checked)
                    {
                        string desktopFolder = Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory);
                        CreateShortcut(Path.Combine(desktopFolder, "ANTAI Sentinel.lnk"), exePath, targetDir);
                    }

                    // Create Start Menu Shortcut
                    if (chkStartMenu.Checked)
                    {
                        string startMenuFolder = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.StartMenu), "Programs");
                        CreateShortcut(Path.Combine(startMenuFolder, "ANTAI Sentinel.lnk"), exePath, targetDir);
                    }

                    this.Invoke((MethodInvoker)delegate
                    {
                        progressBar.Value = 100;
                        lblStatus.Text = "✅ INSTALLAZIONE COMPLETATA CON SUCCESSO!";
                    });

                    // Launch App if checked
                    if (chkLaunch.Checked && File.Exists(exePath))
                    {
                        System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
                        {
                            FileName = exePath,
                            WorkingDirectory = targetDir,
                            UseShellExecute = true
                        });
                    }

                    this.Invoke((MethodInvoker)delegate
                    {
                        MessageBox.Show("ANTAI Sentinel è stato installato con successo sul tuo computer!", "Installazione Completata", MessageBoxButtons.OK, MessageBoxIcon.Information);
                        Application.Exit();
                    });
                }
                catch (Exception ex)
                {
                    this.Invoke((MethodInvoker)delegate
                    {
                        lblStatus.Text = "Errore durante l'installazione.";
                        MessageBox.Show("Errore durante l'installazione: " + ex.Message, "Errore Setup", MessageBoxButtons.OK, MessageBoxIcon.Error);
                        btnInstall.Enabled = true;
                    });
                }
            });
        }

        private void CreateShortcut(string shortcutPath, string targetPath, string workingDirectory)
        {
            try
            {
                Type shellType = Type.GetTypeFromProgID("WScript.Shell");
                dynamic shell = Activator.CreateInstance(shellType);
                var shortcut = shell.CreateShortcut(shortcutPath);
                shortcut.TargetPath = targetPath;
                shortcut.WorkingDirectory = workingDirectory;
                shortcut.Description = "ANTAI — Autonomous AI Cyber Defense Sentinel";
                shortcut.Save();
            }
            catch { }
        }
    }
}
