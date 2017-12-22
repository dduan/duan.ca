---
layout: page
---

<div class="content list">
{% if site.links.size == 0 %}
  <h2>No post found</h2>
{% else %}
{% assign links = site.links | sort: 'date' | reverse %}
{% for link in links %}
  <div class="list-item">
    <h3 class="list-link-title">
      <a href="{{ link.link }}">{{ link.title }}</a>
      <div class="list-link-date">
        <a href="{{ site.baseurl }}{{ link.url }}">
        <small><time>{{ link.date | date_to_string }}</time></small>
        </a>
      </div>
    </h3>
    {% if forloop.index < 6 %}
      {{ link.content }}
      <hr>
    {% endif %}
  </div>
{% endfor %}
{% endif %}
</div>
