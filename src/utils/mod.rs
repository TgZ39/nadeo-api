pub fn to_list(list: &[String]) -> String {
    let mut out = String::new();
    for e in list {
        out.push_str(e);
        out.push(',');
    }
    out.pop();

    out
}

#[macro_export]
macro_rules! execute_into {
    ($client:expr, $request:expr, $format:ty) => {{
        async fn get(client: &mut $crate::NadeoClient, request: $crate::NadeoRequest) -> $crate::Result<$format> {
            let res = client.execute(request).await?;
            let out = res.json::<$format>().await?;

            Ok(out)
        }

        get($client, $request)
    }};
}
