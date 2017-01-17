require 'jekyll/tagging'

module Jekyll
  module LinkBackFilter
    def link_back(input, base, url)
        total = input.scan(",").count
        if total == 0
            return input
        end
        n = (input.length * 0.618 % total).round
        count = 0
        input.gsub(",") do |x|
            dot = x
            if n == count
                dot = "<a class=\"backlink\" rel=\"canonical\" title=\"From dduan.net\" href=#{base}#{url}>,</a>"
            end
            count += 1
            dot
        end
    end
  end
end

Liquid::Template.register_filter(Jekyll::LinkBackFilter)
