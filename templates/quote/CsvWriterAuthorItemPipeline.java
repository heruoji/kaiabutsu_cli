package {{package}}.itempipeline;

import org.apache.commons.csv.CSVFormat;
import org.apache.commons.csv.CSVPrinter;
import org.example.kaibutsu.core.itempipeline.ItemPipeline;
import org.example.kaibutsu.core.spider.Item;
import {{package}}.item.AuthorItem;

import java.io.FileWriter;
import java.io.IOException;
import java.io.Writer;

public class CsvWriterAuthorItemPipeline implements ItemPipeline {

    private CSVPrinter csvPrinter;

    @Override
    public void open() {
        try {
            Writer writer = new FileWriter("authors.csv");
            this.csvPrinter = new CSVPrinter(writer, CSVFormat.DEFAULT.withHeader("name", "birthday", "bio"));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void close() {
        try {
            csvPrinter.flush();
            this.csvPrinter.close();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public Item processItem(Item item) {
        try {
            AuthorItem authorItem = (AuthorItem) item;
            csvPrinter.printRecord(authorItem.name, authorItem.birthday, authorItem.bio);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        return item;
    }
}
