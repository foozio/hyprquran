import urllib.request
import json
import ssl

def main():
    url = "https://cdn.jsdelivr.net/npm/quran-json@3.1.2/dist/quran.json"
    
    # Create an unverified context to avoid SSL verification issues in some environments
    context = ssl._create_unverified_context()
    
    try:
        print(f"Downloading from {url}...")
        with urllib.request.urlopen(url, context=context) as response:
            data = json.loads(response.read().decode('utf-8'))
            
        print("Download complete. Converting...")
        
        with open("quran-uthmani.txt", "w", encoding="utf-8") as f:
            for surah in data:
                surah_id = surah['id']
                for verse in surah['verses']:
                    verse_id = verse['id']
                    text = verse['text']
                    # Clean the text if necessary, ensure no pipes are in the text (rare)
                    text = text.replace('|', '')
                    f.write(f"{surah_id}|{verse_id}|{text}\n")
                    
        print("Conversion complete. Saved to quran-uthmani.txt")
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()

