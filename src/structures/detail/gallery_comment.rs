use chrono::DateTime;
use regex::Regex;
use visdom::Vis;
use crate::utils::{parse_u32, parse_u64};

#[derive(Debug, PartialEq)]
pub struct GalleryComment {
    /// 0 for uploader comment. cannot vote.
    pub id: u64,
    /// uploader comment is `None`.
    pub score_opt: Option<u32>,
    pub editable: bool,
    pub vote_up_able: bool,
    pub vote_up_ed: bool,
    pub vote_down_able: bool,
    pub vote_down_ed: bool,
    pub is_uploader: bool,
    /// uploader comment is `None`.
    pub vote_state_opt: Option<String>,
    pub posted_timestamp: i64,
    pub user: String,
    pub comment: String,
    pub last_edited_timestamp_opt: Option<i64>,
}

impl ToString for GalleryComment {
    fn to_string(&self) -> String {
        if let Some(score) = self.score_opt {
            format!("[{}]{}:{}", score, self.user, self.comment)
        } else {
            format!("[uploader]{}:{}", self.user, self.comment)
        }
    }
}

impl GalleryComment {
    /// 1. Uploader Comment
    /// ```html
    /// <!-- uploader comment -->
    /// <a name="c0"></a>
    /// <div class="c1">
    ///     <div class="c2">
    ///         <div class="c3">Posted on 02 July 2019, 11:50 by: &nbsp; <a
    ///                 href="https:///e-hentai.org/uploader/qq3870990">qq3870990</a>&nbsp; &nbsp; <a
    ///                 href="https:///forums.e-hentai.org/index.php?showuser=1725168"><img class="ygm"
    ///                     src="https:///ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
    ///         <div class="c4 nosel"><a name="ulcomment"></a>Uploader Comment</div>
    ///         <div class="c"></div>
    ///     </div>
    ///     <div class="c6" id="comment_0">
    ///         =========================================================<br>不咕鸟欢迎各位甲方大佬委托汉化本子<br>感谢淘宝“涩谷成人玩具”对本组的大力赞助，有意向的可以去店内逛逛，多多关注。<br>备注咕咕咕有优惠<br><br>详情请联系：2820261867<br>特别注明：<br><br>禁止删除水印封面进行转载，禁止不带汉化组名进行转载，尤其是哔咔，再发现类似情况，外流版本将所有页全部打上水印，无水印版本只提供给金主。<br><br>=======================================================<br><br>RAW：<a
    ///             href="https:///e-hentai.org/g/1378957/7f626bf1d2/">https:///e-hentai.org/g/1378957/7f626bf1d2/</a></div>
    ///     <div class="c7" id="cvotes_0" style="display:none"></div>
    /// </div>
    /// ```
    /// Or
    /// 2. Others Comment
    /// ```html
    /// <a name="c3922745"></a>
    /// <div class="c1">
    ///     <div class="c2">
    ///         <div class="c3">Posted on 24 September 2020, 09:55 by: &nbsp; <a
    ///                 href="https://e-hentai.org/uploader/Kalinkawow">Kalinkawow</a>&nbsp; &nbsp; <a
    ///                 href="https://forums.e-hentai.org/index.php?showuser=4997064"><img class="ygm"
    ///                     src="https://ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
    ///         <div class="c4 nosel">[<a id="comment_vote_up_3922745" style="" href="#"
    ///                 onclick="vote_comment_up(3922745); this.blur(); return false">Vote+</a>] &nbsp; [<a
    ///                 id="comment_vote_down_3922745" style="" href="#"
    ///                 onclick="vote_comment_down(3922745); this.blur(); return false">Vote-</a>]</div>
    ///         <div class="c5 nosel" onmouseover="document.getElementById('cvotes_3922745').style.display=''"
    ///             onclick="this.onmouseover(); this.onmouseout=undefined"
    ///             onmouseout="document.getElementById('cvotes_3922745').style.display='none'">Score <span
    ///                 id="comment_score_3922745" style="opacity:1.0">+257</span></div>
    ///         <div class="c"></div>
    ///     </div>
    ///     <div class="c6" id="comment_3922745">猎 妈 人</div>
    ///     <div class="c7" id="cvotes_3922745" style="display:none">Base +3, <span>q171718988 +3</span>, <span>Igarashi
    ///             Shioya +6</span>, <span>suhaotian +6</span>, <span>as390393473 +2</span>, <span>Subara45 +4</span>,
    ///         <span>louis friend +6</span>, <span>52wy1314 +6</span>, <span>随缘的亚子 +6</span>, <span>Tchami_zz +2</span>,
    ///         <span>sakkijarven +2</span>, <span>无证萝莉控 +6</span>, <span>DaweiX +4</span>, and 38 more...</div>
    /// </div>
    /// ```
    pub fn parse(ele: &str) -> Result<GalleryComment, String> {
        const PATTERN_COMMENT_ID: &str = r#"<a name="c(\d+)"></a>"#;
        const PATTERN_COMMENT_DATETIME: &str = r#"Posted\s*on\s*(.+?)\s*by"#;

        let root = Vis::load(ele).map_err(|_| String::from("parses gallery comment fail."))?;

        let regex = Regex::new(PATTERN_COMMENT_ID).unwrap();
        let captures = regex.captures(ele).ok_or(String::from("parses gallery comment fail."))?;

        // c0 is uploader comment. cannot vote.
        // id.
        let id = parse_u64(&captures[1])?;

        let c3 = root.find(".c3");
        let posted = c3.text();

        // posted_timestamp.
        let regex = Regex::new(PATTERN_COMMENT_DATETIME).unwrap();
        let captures = regex.captures(&posted).ok_or(String::from("parses gallery comment fail."))?;

        let fmt = "%d %B %Y, %H:%M:%S%.3f %z";
        let date_str = format!("{}:00.000 +0000", &captures[1]);
        let datetime = DateTime::parse_from_str(&date_str, fmt).unwrap();
        let posted_timestamp = datetime.timestamp();

        // user.
        let a = c3.children("a");
        let user = a.text();

        // comment.
        let c6 = root.find(".c6");
        let comment = c6.html();

        // last_edited_timestamp_opt.
        let c8 = root.find(".c8");
        let last_edited_timestamp_opt = if !c8.is_empty() { Some(posted_timestamp) } else { None };

        // is_uploader.
        let c4 = root.find(".c4");
        let is_uploader = c4.text() == "Uploader Comment";

        let (
            mut vote_up_able,
            mut vote_up_ed,
            mut vote_down_able,
            mut vote_down_ed,
            mut editable,
            mut vote_state_opt,
            mut score_opt,
        ) = (false, false, false, false, false, None, None);

        if !is_uploader {
            for a in c4.children("a") {
                let text = a.text();
                if let Some(style) = a.get_attribute("style") {
                    match text.as_str() {
                        "Vote+" => {
                            // vote_up_able, vote_up_ed.
                            vote_up_able = true;
                            vote_up_ed = !style.to_string().is_empty();
                        }
                        "Vote-" => {
                            // vote_down_able, vote_down_ed.
                            vote_down_able = true;
                            vote_down_ed = !style.to_string().is_empty();
                        }
                        _ => {}
                    }
                } else if text == "Edit" {
                    // editable.
                    editable = true;
                }
            }

            // vote_state_opt.
            let c7 = root.find(".c7");
            vote_state_opt = Some(c7.text());

            // score_opt.
            let span = root.find(&format!(r#".c5 #comment_score_{}"#, id));
            score_opt = Some(parse_u32(&span.text()[1..])?);
        }


        Ok(GalleryComment {
            id,
            score_opt,
            editable,
            vote_up_able,
            vote_up_ed,
            vote_down_able,
            vote_down_ed,
            is_uploader,
            vote_state_opt,
            posted_timestamp,
            user,
            comment,
            last_edited_timestamp_opt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        // uploader.
        let ele = r#"
            <a name="c0"></a>
            <div class="c1">
                <div class="c2">
                    <div class="c3">Posted on 02 July 2019, 11:50 by: &nbsp; <a
                            href="https:///e-hentai.org/uploader/qq3870990">qq3870990</a>&nbsp; &nbsp; <a
                            href="https:///forums.e-hentai.org/index.php?showuser=1725168"><img class="ygm"
                                src="https:///ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
                    <div class="c4 nosel"><a name="ulcomment"></a>Uploader Comment</div>
                    <div class="c"></div>
                </div>
                <div class="c6" id="comment_0">
                    =========================================================<br>不咕鸟欢迎各位甲方大佬委托汉化本子<br>感谢淘宝“涩谷成人玩具”对本组的大力赞助，有意向的可以去店内逛逛，多多关注。<br>备注咕咕咕有优惠<br><br>详情请联系：2820261867<br>特别注明：<br><br>禁止删除水印封面进行转载，禁止不带汉化组名进行转载，尤其是哔咔，再发现类似情况，外流版本将所有页全部打上水印，无水印版本只提供给金主。<br><br>=======================================================<br><br>RAW：<a
                        href="https:///e-hentai.org/g/1378957/7f626bf1d2/">https:///e-hentai.org/g/1378957/7f626bf1d2/</a></div>
                <div class="c7" id="cvotes_0" style="display:none"></div>
            </div>
        "#;
        assert_eq!(GalleryComment::parse(ele).is_ok(), true);

        // others.
        let ele = r##"
            <a name="c3922745"></a>
            <div class="c1">
                <div class="c2">
                    <div class="c3">Posted on 24 September 2020, 09:55 by: &nbsp; <a
                            href="https://e-hentai.org/uploader/Kalinkawow">Kalinkawow</a>&nbsp; &nbsp; <a
                            href="https://forums.e-hentai.org/index.php?showuser=4997064"><img class="ygm"
                                src="https://ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
                    <div class="c4 nosel">[<a id="comment_vote_up_3922745" style="" href="#"
                            onclick="vote_comment_up(3922745); this.blur(); return false">Vote+</a>] &nbsp; [<a
                            id="comment_vote_down_3922745" style="" href="#"
                            onclick="vote_comment_down(3922745); this.blur(); return false">Vote-</a>]</div>
                    <div class="c5 nosel" onmouseover="document.getElementById('cvotes_3922745').style.display=''"
                        onclick="this.onmouseover(); this.onmouseout=undefined"
                        onmouseout="document.getElementById('cvotes_3922745').style.display='none'">Score <span
                            id="comment_score_3922745" style="opacity:1.0">+257</span></div>
                    <div class="c"></div>
                </div>
                <div class="c6" id="comment_3922745">猎 妈 人</div>
                <div class="c7" id="cvotes_3922745" style="display:none">Base +3, <span>q171718988 +3</span>, <span>Igarashi
                        Shioya +6</span>, <span>suhaotian +6</span>, <span>as390393473 +2</span>, <span>Subara45 +4</span>,
                    <span>louis friend +6</span>, <span>52wy1314 +6</span>, <span>随缘的亚子 +6</span>, <span>Tchami_zz +2</span>,
                    <span>sakkijarven +2</span>, <span>无证萝莉控 +6</span>, <span>DaweiX +4</span>, and 38 more...</div>
            </div>
        "##;
        assert_eq!(GalleryComment::parse(ele).is_ok(), true);
    }
}
