
markup::define! {
  H1<'a>(text: &'a str) {
    h1[class="font-bold text-2xl"] {
      @markup::raw(text)
    }
  }

  H2<'a>(text: &'a str) {
    h2[class="font-bold text-xl"] {
      @markup::raw(text)
    }
  }

  H3<'a>(text: &'a str) {
    h3[class="font-bold text-lg"] {
      @markup::raw(text)
    }
  }

  Paragraph<'a>(text: &'a str) {
    p[class="my-4 text-serif"] {
      @markup::raw(text)
    }
  }

  Code<'a>(text: &'a str) {
    pre {
      code {
        @text
      }
    }
  }

  ListItem<'a>(text: &'a str) {
    li {
      @text
    }
  }

  UnorderedList<'a>(items: Vec<&'a str>) {
    ul[class="ml-4 list-disc"] {
      @for item in items.iter() {
        @ListItem {
          text: item
        }
      }
    } 
  }

  OrderedList<'a>(items: Vec<&'a str>) {
    ol[class="ml-4 list-decimal"] {
      @for item in items.iter() {
        @ListItem {
          text: item
        }
      }
    } 
  }

  Quote<'a>(text: &'a str) {
    p[class="text-lg m-3 italic"] {
      @text
    }
  }
}