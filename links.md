---
layout: page
---

<div class="content list">
{% if site.links.size == 0 %}
  <h2>No post found</h2>
{% else %}
{% for link in site.links %}
  <div class="list-item">
    <h3 class="list-link-title">
      <a href="{{ link.link }}">{{ link.title }}</a>
      <div class="list-link-date">
        <a href="{{ site.baseurl }}{{ link.url }}">
        <small><time>{{ link.date | date_to_string }}</time></small>
        </a>
      </div>
    </h3>
  </div>
  {% if forloop.index < 6 %}
  {{ link.content }}
  {% endif %}
{% endfor %}
{% endif %}
</div>
