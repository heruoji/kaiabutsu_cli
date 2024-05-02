package {{package}}.spider;

import org.example.kaibutsu.core.downloader.Request;
import org.example.kaibutsu.core.downloader.Response;
import org.example.kaibutsu.core.spider.Spider;
import org.example.kaibutsu.core.spider.SpiderResponse;
import {{package}}.item.AuthorItem;

import java.util.List;
import java.util.stream.Collectors;

public class QuoteSpider implements Spider {

    public Request startRequest() {
        return new Request("https://quotes.toscrape.com", "parseMain");
    }

    public SpiderResponse parseMain(Response response, SpiderResponse.SpiderResponseBuilder builder) {
        List<Request> authorRequests = response.select(".author + a").stream().map(link -> new Request(link.absUrl("href"), "parseAuthor")).collect(Collectors.toList());
        List<Request> paginationRequests = response.select("li.next a").stream().map(link -> new Request(link.absUrl("href"), "parseMain")).toList();
        authorRequests.addAll(paginationRequests);

        return builder.requests(authorRequests).build();
    }

    public SpiderResponse parseAuthor(Response response, SpiderResponse.SpiderResponseBuilder builder) {
        String name = response.select("h3.author-title").text();
        String birthday = response.select(".author-born-date").text();
        String bio = response.select(".author-description").text();
        AuthorItem item = new AuthorItem();
        item.name = name;
        item.birthday = birthday;
        item.bio = bio;
        return builder.addItem(item).build();
    }
}

