import org.json.simple.JSONArray;
import org.json.simple.JSONObject;
import org.json.simple.parser.JSONParser;

import java.net.HttpURLConnection;
import java.net.URL;
import java.util.Scanner;

public class API {
    public static void main(String[] args){
        try{
            URL url = new URL("https://www.metaweather.com/api/location/search/?query=london");

            HttpURLConnection conn = (HttpURLConnection) url.openConnection();
            conn.setRequestMethod("GET");
            conn.connect();

            int responseCode = conn.getResponseCode();
            if (responseCode != 200) {
                throw new RuntimeException("Incorrect HTTP response. Expected 200. Received: " + responseCode);
            }

            StringBuilder informationString = new StringBuilder();
            Scanner scanner = new Scanner(url.openStream());
            while (scanner.hasNext()){
                informationString.append(scanner.nextLine());
            }

            //Close and show response
            scanner.close();
            System.out.println(informationString);

            JSONParser parse = new JSONParser();
            JSONArray dataObject = (JSONArray) parse.parse(String.valueOf(informationString));

            // Show weather data
            JSONObject countryData = (JSONObject) dataObject.get(0);
            System.out.println("Location Type: " + countryData.get("location_type"));
            System.out.println("Location Name: " + countryData.get("title"));
            System.out.println("Coordinates: " + countryData.get("latt_long"));
        }
        catch (Exception e){
            e.printStackTrace();
        }
    }
}

