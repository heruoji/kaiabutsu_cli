package {{package}}.item;


import org.example.kaibutsu.core.spider.Item;

public class AuthorItem implements Item {
    public String name;
    public String birthday;
    public String bio;

    @Override
    public String toString() {
        return String.format("{ name : %s, birthday : %s, bio : %s }", name, birthday, bio);
    }
}
