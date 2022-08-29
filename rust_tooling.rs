<mxfile host="app.diagrams.net" modified="2022-08-29T12:21:37.502Z" agent="5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36" etag="_oel4erGGYyhcO38CbDW" version="20.2.7" type="github" pages="4">
  <diagram id="6hkav5tRjC0ZOaah69UK" name="development">
    <mxGraphModel dx="1550" dy="835" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="827" pageHeight="1169" math="0" shadow="0">
      <root>
        <mxCell id="0" />
        <mxCell id="1" parent="0" />
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-4" value="" style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;" edge="1" parent="1" source="8wuzUHHyvKwM_tFGuoH8-2">
          <mxGeometry relative="1" as="geometry">
            <mxPoint x="390" y="190" as="targetPoint" />
          </mxGeometry>
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-2" value="static analysis" style="rounded=1;whiteSpace=wrap;html=1;" vertex="1" parent="1">
          <mxGeometry x="190" y="160" width="120" height="60" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-5" value="lint: clippy" style="rounded=1;whiteSpace=wrap;html=1;" vertex="1" parent="1">
          <mxGeometry x="390" y="160" width="120" height="60" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-9" value="" style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;" edge="1" parent="1" source="8wuzUHHyvKwM_tFGuoH8-7" target="8wuzUHHyvKwM_tFGuoH8-8">
          <mxGeometry relative="1" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-7" value="dynamic analysis" style="rounded=1;whiteSpace=wrap;html=1;" vertex="1" parent="1">
          <mxGeometry x="190" y="240" width="120" height="60" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-8" value="valgrind" style="rounded=1;whiteSpace=wrap;html=1;" vertex="1" parent="1">
          <mxGeometry x="390" y="240" width="120" height="60" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-12" value="" style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;" edge="1" parent="1" source="8wuzUHHyvKwM_tFGuoH8-10" target="8wuzUHHyvKwM_tFGuoH8-11">
          <mxGeometry relative="1" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-14" style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;entryX=0;entryY=0.5;entryDx=0;entryDy=0;" edge="1" parent="1" source="8wuzUHHyvKwM_tFGuoH8-10" target="8wuzUHHyvKwM_tFGuoH8-13">
          <mxGeometry relative="1" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-19" style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;entryX=0;entryY=0.5;entryDx=0;entryDy=0;" edge="1" parent="1" source="8wuzUHHyvKwM_tFGuoH8-10" target="8wuzUHHyvKwM_tFGuoH8-18">
          <mxGeometry relative="1" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-10" value="benchmark" style="rounded=1;whiteSpace=wrap;html=1;" vertex="1" parent="1">
          <mxGeometry x="190" y="330" width="120" height="60" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-11" value="runtime: criterion" style="whiteSpace=wrap;html=1;rounded=1;" vertex="1" parent="1">
          <mxGeometry x="390" y="330" width="120" height="60" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-13" value="heap: jemalloc-ctl" style="whiteSpace=wrap;html=1;rounded=1;" vertex="1" parent="1">
          <mxGeometry x="390" y="400" width="120" height="60" as="geometry" />
        </mxCell>
        <mxCell id="8wuzUHHyvKwM_tFGuoH8-18" value="stack: ??" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#f8cecc;strokeColor=#b85450;" vertex="1" parent="1">
          <mxGeometry x="390" y="480" width="120" height="60" as="geometry" />
        </mxCell>
      </root>
    </mxGraphModel>
  </diagram>
  <diagram id="FCLnhBwmY0QxwDr8iMeC" name="testing">
    <mxGraphModel dx="1550" dy="835" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="827" pageHeight="1169" math="0" shadow="0">
      <root>
        <mxCell id="0" />
        <mxCell id="1" parent="0" />
      </root>
    </mxGraphModel>
  </diagram>
  <diagram id="gTa6KhZeBXI5pYnxfB4H" name="production">
    <mxGraphModel dx="1550" dy="835" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="827" pageHeight="1169" math="0" shadow="0">
      <root>
        <mxCell id="0" />
        <mxCell id="1" parent="0" />
      </root>
    </mxGraphModel>
  </diagram>
  <diagram id="CPZ0bw007odab0nsNT1O" name="build">
    <mxGraphModel dx="1550" dy="835" grid="1" gridSize="10" guides="1" tooltips="1" connect="1" arrows="1" fold="1" page="1" pageScale="1" pageWidth="827" pageHeight="1169" math="0" shadow="0">
      <root>
        <mxCell id="0" />
        <mxCell id="1" parent="0" />
      </root>
    </mxGraphModel>
  </diagram>
</mxfile>
