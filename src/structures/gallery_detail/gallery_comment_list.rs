use std::iter::zip;
use visdom::Vis;
use crate::{EhResult, Parser, structures::gallery_detail::GalleryComment};

#[derive(Debug, PartialEq)]
pub struct GalleryCommentList {
    pub comment_vec: Vec<GalleryComment>,
    pub has_more: bool,
}

impl Parser for GalleryCommentList {
    /// ```html
    /// <div id="cdiv" class="gm">
    ///     <!-- uploader comment -->
    ///     <a name="c0"></a>
    ///     <div class="c1">...</div>
    ///
    ///     <a name="c3054522"></a>
    ///     <div class="c1">...</div>
    ///
    ///     <div id="chd">
    ///         <p>There is 1 more comment below the viewing threshold - <a
    ///             href="https://e-hentai.org/g/1740161/b90e67b628/?hc=1#comments" rel="nofollow">click to show all</a>.
    ///         </p>
    ///         <p id="postnewcomment">[<a href="#"
    ///                 onclick="display_comment_field(); document.getElementById('postnewcomment').style.display='none'; return false">Post
    ///                 New Comment</a>]</p>
    ///     </div>
    ///     <a name="cnew"></a>
    ///     <div id="formdiv" style="display:none">
    ///         <form method="post" action="#cnew">
    ///             <textarea name="commenttext_new"
    ///                 placeholder="Enter your comments here, then hit Post Comment. If the last comment posted is yours, this will be appended to that post."></textarea>
    ///             <p><input type="submit" value="Post Comment"></p>
    ///         </form>
    ///     </div>
    /// </div>
    /// ```
    fn parse(doc: &str) -> EhResult<Self> {
        let root = Vis::load(doc)?;

        let mut comment_vec = Vec::new();
        let cas = root.find(r#"a[name^=c][name!=cnew]"#);
        let c1s = root.find(".c1");

        for (ca, c1) in zip(cas, c1s) {
            let combine = &format!("{}{}", ca.outer_html(), c1.outer_html());
            let comment = GalleryComment::parse(&combine)?;
            comment_vec.push(comment);
        }

        let show_all = root.find("#chd [rel=nofollow]");
        let has_more = !show_all.is_empty();

        Ok(GalleryCommentList {
            comment_vec,
            has_more,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r##"
            <div id="cdiv" class="gm">
                <a name="c0"></a>
                <div class="c1">
                    <div class="c2">
                        <div class="c3">Posted on 23 September 2020, 14:04 by: &nbsp; <a
                                href="https://e-hentai.org/uploader/qq3870990">qq3870990</a>&nbsp; &nbsp; <a
                                href="https://forums.e-hentai.org/index.php?showuser=1725168"><img class="ygm"
                                    src="https://ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
                        <div class="c4 nosel"><a name="ulcomment"></a>Uploader Comment</div>
                        <div class="c"></div>
                    </div>
                    <div class="c6" id="comment_0">RAW???<a
                            href="https://e-hentai.org/g/1511310/8e568fd1b0/">https://e-hentai.org/g/1511310/8e568fd1b0/</a><br><br>????????????
                        ?????????????????????
                        ????????????<br><br>??????????????????????????????????????????????????????????????????????????????????????????????????????????????????<br>???????????????????????????????????????<br><br>=========================================================<br><br>????????????????????????<br><br>?????????????????????<br>???????????????????????????N1?????????????????????????????????????????????????????????<br><br>??????????????????????????????????????????????????????????????????????????????????????????????????????<br>????????????????????????????????????????????????????????????<br><br>?????????????????????????????????????????????????????????<br>?????????????????????qq???2820261867<br><br>=========================================================
                    </div>
                    <div class="c7" id="cvotes_0" style="display:none"></div>
                </div>

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
                    <div class="c6" id="comment_3922745">??? ??? ???</div>
                    <div class="c7" id="cvotes_3922745" style="display:none">Base +3, <span>q171718988 +3</span>, <span>Igarashi
                            Shioya +6</span>, <span>suhaotian +6</span>, <span>as390393473 +2</span>, <span>Subara45 +4</span>,
                        <span>louis friend +6</span>, <span>52wy1314 +6</span>, <span>??????????????? +6</span>, <span>Tchami_zz +2</span>,
                        <span>sakkijarven +2</span>, <span>??????????????? +6</span>, <span>DaweiX +4</span>, and 38 more...</div>
                </div>
                <div id="chd">
                    <p>There is 1 more comment below the viewing threshold - <a
                            href="https://e-hentai.org/g/1740161/b90e67b628/?hc=1#comments" rel="nofollow">click to show all</a>.
                    </p>
                    <p id="postnewcomment">[<a href="#"
                            onclick="display_comment_field(); document.getElementById('postnewcomment').style.display='none'; return false">Post
                            New Comment</a>]</p>
                </div>
                <a name="cnew"></a>
                <div id="formdiv" style="display:none">
                    <form method="post" action="#cnew">
                        <textarea name="commenttext_new"
                            placeholder="Enter your comments here, then hit Post Comment. If the last comment posted is yours, this will be appended to that post."></textarea>
                        <p><input type="submit" value="Post Comment"></p>
                    </form>
                </div>
            </div>
        "##;

        assert_eq!(GalleryCommentList::parse(ele).is_ok(), true);
    }
}
