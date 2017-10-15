(function() {var implementors = {};
implementors["quote_bot"] = ["impl&lt;'a, 'insert, DB&gt; <a class=\"trait\" href=\"diesel/insertable/trait.Insertable.html\" title=\"trait diesel::insertable::Insertable\">Insertable</a>&lt;<a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.table.html\" title=\"struct quote_bot::db::schema::quotes::table\">table</a>, DB&gt; for &amp;'insert <a class=\"struct\" href=\"quote_bot/db/models/struct.NewQuote.html\" title=\"struct quote_bot::db::models::NewQuote\">NewQuote</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: <a class=\"trait\" href=\"diesel/backend/trait.Backend.html\" title=\"trait diesel::backend::Backend\">Backend</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>ColumnInsertValue&lt;<a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.message_id.html\" title=\"struct quote_bot::db::schema::quotes::message_id\">message_id</a>, <a class=\"type\" href=\"diesel/expression/helper_types/type.AsExpr.html\" title=\"type diesel::expression::helper_types::AsExpr\">AsExpr</a>&lt;&amp;'insert &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.message_id.html\" title=\"struct quote_bot::db::schema::quotes::message_id\">message_id</a>&gt;&gt;, ColumnInsertValue&lt;<a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.quote.html\" title=\"struct quote_bot::db::schema::quotes::quote\">quote</a>, <a class=\"type\" href=\"diesel/expression/helper_types/type.AsExpr.html\" title=\"type diesel::expression::helper_types::AsExpr\">AsExpr</a>&lt;&amp;'insert &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.quote.html\" title=\"struct quote_bot::db::schema::quotes::quote\">quote</a>&gt;&gt;, ColumnInsertValue&lt;<a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.created_by_id.html\" title=\"struct quote_bot::db::schema::quotes::created_by_id\">created_by_id</a>, <a class=\"type\" href=\"diesel/expression/helper_types/type.AsExpr.html\" title=\"type diesel::expression::helper_types::AsExpr\">AsExpr</a>&lt;&amp;'insert &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.created_by_id.html\" title=\"struct quote_bot::db::schema::quotes::created_by_id\">created_by_id</a>&gt;&gt;, ColumnInsertValue&lt;<a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.quoted_by_id.html\" title=\"struct quote_bot::db::schema::quotes::quoted_by_id\">quoted_by_id</a>, <a class=\"type\" href=\"diesel/expression/helper_types/type.AsExpr.html\" title=\"type diesel::expression::helper_types::AsExpr\">AsExpr</a>&lt;&amp;'insert &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.quoted_by_id.html\" title=\"struct quote_bot::db::schema::quotes::quoted_by_id\">quoted_by_id</a>&gt;&gt;, ColumnInsertValue&lt;<a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.guild_id.html\" title=\"struct quote_bot::db::schema::quotes::guild_id\">guild_id</a>, <a class=\"type\" href=\"diesel/expression/helper_types/type.AsExpr.html\" title=\"type diesel::expression::helper_types::AsExpr\">AsExpr</a>&lt;&amp;'insert &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <a class=\"struct\" href=\"quote_bot/db/schema/quotes/struct.guild_id.html\" title=\"struct quote_bot::db::schema::quotes::guild_id\">guild_id</a>&gt;&gt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>: InsertValues&lt;DB&gt;,&nbsp;</span>","impl&lt;'a, 'insert, DB&gt; <a class=\"trait\" href=\"diesel/insertable/trait.Insertable.html\" title=\"trait diesel::insertable::Insertable\">Insertable</a>&lt;<a class=\"struct\" href=\"quote_bot/db/schema/authors/struct.table.html\" title=\"struct quote_bot::db::schema::authors::table\">table</a>, DB&gt; for &amp;'insert <a class=\"struct\" href=\"quote_bot/db/models/struct.NewAuthor.html\" title=\"struct quote_bot::db::models::NewAuthor\">NewAuthor</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DB: <a class=\"trait\" href=\"diesel/backend/trait.Backend.html\" title=\"trait diesel::backend::Backend\">Backend</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>ColumnInsertValue&lt;<a class=\"struct\" href=\"quote_bot/db/schema/authors/struct.id.html\" title=\"struct quote_bot::db::schema::authors::id\">id</a>, <a class=\"type\" href=\"diesel/expression/helper_types/type.AsExpr.html\" title=\"type diesel::expression::helper_types::AsExpr\">AsExpr</a>&lt;&amp;'insert &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, <a class=\"struct\" href=\"quote_bot/db/schema/authors/struct.id.html\" title=\"struct quote_bot::db::schema::authors::id\">id</a>&gt;&gt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">,)</a>: InsertValues&lt;DB&gt;,&nbsp;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
