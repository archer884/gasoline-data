select * from fillup f
    join "user" u on f.user_id = u.id

where u.id = $1
order by f.id
offset $2
limit $3
