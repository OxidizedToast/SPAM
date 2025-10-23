using System;
using System.IO;
namespace spam {
    public class Status {
        public static void CheckStatus() {
            // Properly expand home directory
            string home = Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
            string filePath = Path.Combine(home, ".config", "spam.txt");

            // Ensure the folder exists
            Directory.CreateDirectory(Path.GetDirectoryName(filePath));

            if (File.Exists(filePath)) {
                string fileContents = File.ReadAllText(filePath);
                bool loadedStatus = bool.Parse(fileContents);
                string status = loadedStatus ? "on" : "off";
                Console.WriteLine($"Status is {status}");
            } else {
                // Create the file with default value false
                using (var fs = File.Create(filePath)) { } // just close immediately
                File.WriteAllText(filePath, "false");
                Console.WriteLine("Status file created with default 'off'");
            }
        }
    }
}

